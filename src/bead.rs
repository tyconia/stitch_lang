//! A bead is a shorthand for a node table. A bead has dynamic sets
//! of inputs and outputs called [`Slots`]s

use crate::prelude::*;
use serde::*;

pub type BeadParam = HashMap<String, Slot>;
pub type BeadArg = Vec<(String, SlotArg)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bead {
    pub title: String,
    pub description: String,
    pub archetype: BeadArchetype,

    pub inputs: BeadParam,
    pub outputs: BeadParam,
    pub implement: Execute,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BeadArchetype {
    // provides values in diagnostic mode
    Tester,
    // basic input - output node
    Transformation,
    // like transformation but with green threads
    Logical,
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
