use crate::prelude::*;

#[cfg(not(feature = "bevy"))]
pub struct ImplementFn(pub Box<dyn Fn(BeadArg) -> BeadArg>);

#[cfg(not(feature = "bevy"))]
unsafe impl Send for ImplementFn {}

#[cfg(not(feature = "bevy"))]
pub struct Runtime {
    tick: u128,
    implements: HashMap<Uuid, ImplementFn>,
    beads: HashMap<Uuid, Bead>,
    receiver: mpsc::Receiver<RuntimeReception>,
}

#[cfg(not(feature = "bevy"))]
#[derive(Debug)]
pub enum RuntimeReception {
    Arg(SlotArg),
    Startup,
    Shutdown,
}

#[cfg(not(feature = "bevy"))]
impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // You can choose to exclude field3 or handle it differently
        f.debug_struct("Runtime")
            .field("tick", &self.tick)
            .field("implements", &self.implements.keys())
            .field("beads", &self.beads)
            .finish()
    }
}

#[cfg(not(feature = "bevy"))]
impl Runtime {
    pub fn new() -> (Self, mpsc::Sender<RuntimeReception>) {
        let (tx, rx) = mpsc::channel(usize::MAX / 8);

        let rt = Self {
            tick: 0,
            implements: HashMap::default(),
            beads: HashMap::default(),
            receiver: rx,
        };

        (rt, tx)
    }
    pub async fn run(mut self) -> Duration {
        let now = Instant::now();
        log::info!("start");

        while let Some(event) = self.receiver.recv().await {
            self.tick += 1;

            match event {
                RuntimeReception::Startup => {
                    log::debug!("startup event received");
                }
                RuntimeReception::Arg(arg) => {
                    log::debug!("arg received: {:?}", SlotParam::from(arg));
                }
                RuntimeReception::Shutdown => {
                    log::info!("stopped. elapsed: {}s", now.elapsed().as_secs());
                    break;
                }
            }
        }

        now.elapsed()
    }

    pub fn get_tick(&self) -> u128 {
        self.tick
    }

    pub fn register_impl(&mut self, implement: ImplementFn, namespace: String) -> bool {
        let uuid = uuid_str(&namespace);
        self.implements.insert(uuid, implement).is_none()
    }

    pub fn register_bead(&mut self, namespace: String, bead: Bead) -> Uuid {
        let uuid = uuid_str(&namespace);
        self.beads.insert(uuid, bead);
        uuid
    }

    pub fn get_bead_from_id(&self, uuid: &Uuid) -> Option<&Bead> {
        self.beads.get(uuid)
    }

    pub fn get_bead(&self, namespace: &str) -> Option<&Bead> {
        let uuid = uuid_str(&namespace);
        self.beads.get(&uuid)
    }

    pub fn exec(&self, namespace: String, params: BeadArg) -> Result<BeadArg, RuntimeError> {
        let uuid = uuid_str(&namespace);
        self.implements.get(&uuid).map_or(
            Err(RuntimeError::ImplementNotFound { namespace }),
            |implement| Ok(implement.0(params)),
        )
    }
}

use std::time::*;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum RuntimeError {
    NoEntryPoint,
    BeadNotFound { namespace: String },
    ImplementNotFound { namespace: String },
    IncompatibleParams { expected: String, received: String },
}

use std::fmt;

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match self {
            Self::NoEntryPoint => {
                format!("No valid entry point found. Consider adding an Event bead")
            }
            Self::IncompatibleParams { expected, received } => {
                format!("Incompatible parameters. Expected: ${expected}, Received: ${received}")
            }
            Self::ImplementNotFound { namespace } => {
                format!("Implement for bead not found for namespace: ${namespace}")
            }
            Self::BeadNotFound { namespace } => {
                format!("Bead not found for namespace: ${namespace}")
            }
        };

        write!(f, "{}", err)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(feature = "bevy"))]
    fn execute_runtime() {
        use super::*;
        use crate::runtime;

        const LHS: i32 = 6;
        const RHS: i32 = 2;
        const EXPECTED: i32 = 8;

        const OP_L: &'static str = "operand_lhs";
        const OP_R: &'static str = "operand_rhs";

        let (mut rt, tx) = Runtime::new();

        rt.register_impl(
            runtime::ImplementFn(
            Box::new(|params| {
                let lhs = params
                    .iter()
                    .find(|(s, _)| s == OP_L)
                    .unwrap()
                    .1
                    .number()
                    .unwrap();
                let rhs = params
                    .iter()
                    .find(|(s, _)| s == OP_R)
                    .unwrap()
                    .1
                    .number()
                    .unwrap();

                let op_result = lhs + rhs;

                let mut result = BeadArg::default();

                result.push(("result".into(), SlotArg::Number(op_result)));

                result
            })),
            "utils/add".into(),
        );

        let mut params = BeadArg::default();

        params.push(("operand_lhs".into(), SlotArg::Number(LHS)));

        params.push(("operand_rhs".into(), SlotArg::Number(RHS)));

        let add_result = rt
            .exec("utils/add".into(), params)
            .unwrap()
            .iter()
            .find(|s| &s.0 == "result")
            .unwrap()
            .1
            .number()
            .unwrap();

        assert_eq!(add_result, EXPECTED);
    }
}
