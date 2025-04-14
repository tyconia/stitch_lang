//! A fabric defines connections between beads.

use crate::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fabric {
    beads: Vec<Uuid>,
    stitches: Vec<Stitch>,
}

impl Fabric {
    // spawns tasks for event beads
    //pub fn listen(&mut self, rt: &Runtime, beads: &[Uuid]) -> Vec<impl Future> {
    // filter beads that have complete parameters
    //beads
    //    .iter()
    //    // need bead index
    //    .enumerate()
    //    .filter_map(|(bead_index, bead)| {
    //        let bead = rt.get_bead_from_id(bead)?;
    //
    //        bead.inputs
    //            .iter()
    //            .enumerate()
    //            .filter_map(|(bead_slot_index, (label, slot))| {
    //                let bead_slot = (bead_index, *bead_slot_index).into();
    //
    //                let stitch = self
    //                    .stitches
    //                    .iter()
    //                    .find(|stitch| stitch.src == bead_slot)
    //                    .cloned();
    //
    //                Some((bead_slot, stitch))
    //            })
    //            .filter(|(_, stitch)| stitch.is_some())
    //            .map(|(slot, stitch)| (slot, stitch.unwrap()))
    //            .map(|(slot, stitch)| {
    //                let (tx, rx) = tokio::sync::mpsc::channel(8);
    //                let task = tokio::spawn(async move {
    //                    //while let Some(_) =rx.recv().await
    //                });
    //            });
    //
    //
    //        Some(task)
    //    })
    //    .collect()
    //}

    #[cfg(not(feature = "bevy"))]
    pub fn entry_points(&mut self, rt: &Runtime) -> Result<Vec<Uuid>, RuntimeError> {
        let entries: Vec<Uuid> = self
            .beads
            .iter()
            .filter_map(|uuid| {
                let bead = rt.get_bead_from_id(&uuid)?;

                if bead.archetype == BeadArchetype::Event {
                    Some(uuid.clone())
                } else {
                    None
                }
            })
            .collect();

        if entries.is_empty() {
            return Err(RuntimeError::NoEntryPoint);
        }

        Ok(entries)
    }
}

#[cfg(not(feature = "bevy"))]
#[cfg(test)]
mod test {
    //#[test]
    //fn serialize_fabric() {
    //    use super::*;
    //    use crate::*;
    //    use ron::ser::to_string_pretty;
    //    use std::fs;
    //    use std::io::Write;
    //
    //    let (mut rt, tx) = Runtime::new();
    //
    //    rt.register_impl(runtime::ImplementFn(Box::new(|a| (a))), "utils/identity".into());
    //
    //    rt.register_impl(
    //        runtime::ImplementFn(
    //        Box::new(|args| {
    //            let result = args
    //                .iter()
    //                .map(|arg| format!("{:?}", arg.1))
    //                .collect::<String>();
    //            println!("utils/stdout -> {:?}", result);
    //            vec![]
    //        })),
    //        "utils/stdout".into(),
    //    );
    //
    //    let mut inputs = HashMap::default();
    //
    //    inputs.insert(
    //        "input".into(),
    //        Slot {
    //            title: "".into(),
    //            description: "".into(),
    //            status: FieldStatus::Write,
    //            expected_type: SlotParam::String,
    //        },
    //    );
    //
    //    let mut outputs = HashMap::default();
    //    outputs.insert(
    //        "output".into(),
    //        Slot {
    //            title: "".into(),
    //            description: "".into(),
    //            status: FieldStatus::ReadOnly,
    //            expected_type: SlotParam::String,
    //        },
    //    );
    //
    //    let indentity_bead = rt.register_bead(
    //        "identity".into(),
    //        Bead {
    //            title: "Identity".into(),
    //            description: "returns the parameters as is".into(),
    //            archetype: BeadArchetype::Transformation,
    //            inputs,
    //            outputs,
    //            implement: Execute::Std {
    //                src: "utils/identity".into(),
    //            },
    //        },
    //    );
    //
    //    let mut inputs = HashMap::default();
    //
    //    inputs.insert(
    //        "input".into(),
    //        Slot {
    //            title: "".into(),
    //            description: "".into(),
    //            status: FieldStatus::Write,
    //            expected_type: SlotParam::String,
    //        },
    //    );
    //
    //    let print_bead = rt.register_bead(
    //        "print".into(),
    //        Bead {
    //            title: "Printer".into(),
    //            description: "prints value to stdout".into(),
    //            archetype: BeadArchetype::Effect,
    //            inputs,
    //            outputs: HashMap::default(),
    //            implement: Execute::Std {
    //                src: "utils/stdout".into(),
    //            },
    //        },
    //    );
    //
    //    let stitches = [Stitch {
    //        link_type: StichLink::Adjacent,
    //        src: (0, 0).into(),
    //        to: [BeadSlot::from(1)].to_vec(),
    //    }]
    //    .to_vec();
    //
    //    let fabric = Fabric {
    //        beads: [indentity_bead, print_bead].to_vec(),
    //        stitches,
    //    };
    //
    //    utils::to_file(fabric, "fabric.ron");
    //}
}
