//! A bead is a shorthand for a node table. A bead has dynamic sets
//! of inputs and outputs called [`Slots`]s

use crate::prelude::*;
use serde::*;

pub type BeadParam = HashMap<String, Slot>;
pub type BeadArg = Vec<(String, SlotArg)>;

#[cfg(not(feature = "bevy"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bead {
    pub title: String,
    pub description: String,
    pub archetype: BeadArchetype,

    pub inputs: BeadParam,
    pub outputs: BeadParam,
    pub implement: Execute,
}

#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[cfg(feature = "bevy")]
#[derive(Serialize, Deserialize, Debug, Component)]
pub struct Bead {
    pub title: String,
    pub description: String,
    pub archetype: BeadArchetype,

    pub inputs: BeadParam,
    pub outputs: BeadParam,
    pub implement: Execute,
}

// TODO: index must be Entity
// instance of a bead in a fabric
#[cfg(feature = "bevy")]
#[derive(Component)]
pub struct BeadHandle {
    pub index: usize,
    pub bead: Uuid,
}

impl Bead {
    // creates
    pub async fn spool(&mut self, sources: &[Stitch]) {}

    #[cfg(feature = "bevy")]
    pub fn spawn(&self, fabric_parent: &mut ChildBuilder, fabric: &Fabric) {
        // spawn an instance of a bead
        let bead_instance = fabric_parent
            .spawn(BeadHandle {
                index: 0,
                bead: Uuid::new_v5(&Uuid::NAMESPACE_URL, "".to_string().as_bytes()),
            })
            .with_children(|bead_handle| {
                self.inputs.iter().enumerate().map(|(input_index, slot)| {
                    bead_handle.spawn(BeadSlot::from((bead_handle.parent_entity(), input_index)));
                });
            })
            .id();
    }

    #[cfg(feature = "bevy")]
    pub fn spawn_await_inputs(
        &self,
        cmd: &Commands,
        bead: Query<(&BeadHandle, &Stitch)>,
    ) -> Vec<Entity> {
        vec![]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BeadArchetype {
    // provides values in diagnostic mode
    Tester,
    // basic input - output node
    Transformation,
    // entry point of the bead tree, provides a payload of values
    Event,
    // read global / environment state
    Environment,
    // mutations on existing environment or context,
    // any node that contain modifications will automatically
    // be an effect node
    Effect,
}

// The underlying process in the bead
#[derive(Serialize, Deserialize, Debug)]
pub enum Execute {
    Std { src: String },

    Script { src: std::path::PathBuf },

    Fabric(Fabric),
}

#[cfg(test)]
mod tests {
    #[test]
    fn serialize_print() {
        use crate::prelude::*;
        let mut inputs = HashMap::default();

        inputs.insert(
            "input".into(),
            Slot {
                title: "".into(),
                description: "".into(),
                status: FieldStatus::Write,
                expected_type: SlotParam::String,
            },
        );
        let print = Bead {
            title: "Printer".into(),
            description: "prints value to stdout".into(),
            archetype: BeadArchetype::Effect,
            inputs,
            outputs: HashMap::default(),
            implement: Execute::Std {
                src: "stdlib/utils/stdout".into(),
            },
        };

        crate::utils::to_file(print, "src/stdlib/utils/print.ron");
    }

    #[test]
    fn serialize_bead() {
        use super::*;
        use crate::*;
        use ron::ser::to_string_pretty;
        use std::fs;
        use std::io::Write;

        let mut inputs = HashMap::new();
        inputs.insert(
            "operation".into(),
            Slot {
                title: "Operator".into(),
                description: "sets the mode of operation".into(),
                status: FieldStatus::Write,
                expected_type: SlotParam::Choice {
                    options: "+-*/"
                        .chars() // Split string into individual characters
                        .rev()
                        .map(|c| c.to_string()) // Convert each character into a String
                        .collect::<HashSet<String>>(), // Collect into Vec<String>
                },
            },
        );

        inputs.insert(
            "operand_lhs".into(),
            Slot {
                title: "Operand Left side".into(),
                description: "the left hand side of the expression".into(),
                status: FieldStatus::Write,
                expected_type: SlotParam::Number,
            },
        );

        inputs.insert(
            "operand_rhs".into(),
            Slot {
                title: "Operand Right side".into(),
                description: "the right hand side of the expression".into(),
                status: FieldStatus::Write,
                expected_type: SlotParam::Number,
            },
        );

        let mut outputs = HashMap::new();

        outputs.insert(
            "result".into(),
            Slot {
                title: "Arithmetic Result".into(),
                description: "result of the operation".into(),
                status: FieldStatus::ReadOnly,
                expected_type: SlotParam::Number,
            },
        );

        let implement = Execute::Std {
            src: "util/calc".into(),
        };

        let bead = Bead {
            title: "Arithmetic".into(),
            description: "applies operations between 2 operands".into(),
            archetype: BeadArchetype::Transformation,
            inputs,
            outputs,
            implement,
        };

        utils::to_file(&bead, "bead.txt");
    }
}
