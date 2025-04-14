use crate::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct ImplementFn(pub Box<dyn Fn(BeadArg) -> BeadArg>);
unsafe impl Send for ImplementFn {}
unsafe impl Sync for ImplementFn {}

#[derive(Default, Resource)]
pub struct StitchRuntime {
    // a compiled bead
    pub beads: HashMap<Uuid, Bead>,
    // a bead implementation
    pub implements: HashMap<Uuid, ImplementFn>,
}

impl StitchRuntime {
    fn anticipate(&self, fabric: &Fabric) -> StitchPoll {
        StitchPoll(vec![])
    }

    fn execution(&self) {}
}

#[derive(Debug)]
pub struct BeadDependency {
    pub bead: usize,
    pub bead_slot: usize,
}

#[derive(Debug, Resource)]
pub struct StitchSoftCompiled {
    // indicates which beads are executed sequentially
    pub execution_trees: Vec<VecDeque<usize>>,
    // indicates which bead inputs to poll at a time
    pub dependecy_tree: VecDeque<Vec<BeadDependency>>,
}

#[derive(Event, Debug)]
pub struct StitchEvent(pub SlotArg);

#[derive(Debug)]
pub struct StitchPoll(pub Vec<SlotParam>);

impl StitchPoll {
    pub fn expected(&self, arg: SlotArg) -> bool {
        let in_param = SlotParam::from(arg);

        self.0.iter().find(|param| **param == in_param).is_some()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_arg() {
        use crate::prelude::*;
        use crate::bevy_feat::*;
        let poll = StitchPoll([SlotParam::Number].into());

        let expected = poll.expected(SlotArg::Number(22));

        assert!(expected)
    }

    #[test]
    fn validate_arg_unexpected() {
        use crate::prelude::*;
        use crate::bevy_feat::*;
        let poll = StitchPoll([SlotParam::Number].into());

        let expected = poll.expected(SlotArg::String("meow".into()));

        assert_eq!(false, expected)
    }

    #[test]
    fn validate_arg_map() {
        use crate::prelude::*;
        use crate::bevy_feat::*;
        let mut map = HashMap::new();
        map.insert("jump_story".into(), SlotParam::Number);

        let poll = StitchPoll([SlotParam::Map(map)].into());

        let mut map = HashMap::new();
        map.insert("jump_story".into(), SlotArg::Number(12));
        let expected = poll.expected(SlotArg::Map(map));

        assert!(expected)
    }

   #[test]
    fn validate_arg_map_unexpected() {
        use crate::prelude::*;
        use crate::bevy_feat::*;
        let mut map = HashMap::new();
        map.insert("jump_story".into(), SlotParam::Number);

        let poll = StitchPoll([SlotParam::Map(map)].into());

        let mut map = HashMap::new();
        map.insert("jump_storyee".into(), SlotArg::Number(12));
        let expected = poll.expected(SlotArg::Map(map));

        assert_eq!(false, expected)
    }

}
