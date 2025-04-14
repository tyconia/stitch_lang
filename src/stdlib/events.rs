use crate::prelude::*;

pub fn startup() -> Bead {
    let mut outputs = HashMap::default();
    let implement = Execute::Std {
        src: "rt/startup".into(),
    };

    outputs.insert(
        "ready".into(),
        Slot {
            title: "Indicates ready status".into(),
            description: "Bootstraps event nodes".into(),
            status: FieldStatus::ReadOnly,
            expected_type: SlotParam::Bool,
        },
    );

    Bead {
        title: "Startup".into(),
        description: "first event fired up during runtime start".into(),
        archetype: BeadArchetype::Event,
        inputs: HashMap::default(),
        outputs,
        implement,
    }
}
