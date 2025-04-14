use crate::prelude::*;
use serde::*;

#[cfg(not(feature = "bevy"))]
impl From<(usize, usize)> for BeadSlot {
    fn from((bead, slot): (usize, usize)) -> Self {
        Self { bead, slot }
    }
}

#[cfg(not(feature = "bevy"))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BeadSlot {
    bead: usize,
    slot: usize,
}

#[cfg(not(feature = "bevy"))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Stitch {
    pub link_type: StichLink,
    pub src: BeadSlot,
    pub to: BeadSlot,
}

#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[cfg(feature = "bevy")]
impl From<(Entity, usize)> for BeadSlot {
    fn from((bead, slot_index): (Entity, usize)) -> Self {
        Self { bead, slot_index }
    }
}

// TODO: index must be Entity
#[cfg(feature = "bevy")]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub struct BeadSlot {
    bead: Entity,
    slot_index: usize,
}

#[cfg(feature = "bevy")]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Component)]
pub struct Stitch {
    pub link_type: StichLink,
    pub src: BeadSlot,
    pub to: BeadSlot,
}

impl Stitch {
    //pub fn new_connect(src: BeadSlot, to: BeadSlot) -> Self {
    //    unimplemented!()
    //}
    //
    //// adds another destination bead slot
    //pub fn connect(&mut self, to: BeadSlot) {}
    //
    //// changes the source bead slot
    //pub fn change_src(&mut self, src: BeadSlot) {}
    //
    //// checks if source and dest are same types
    //pub fn type_check(src: BeadSlot, to: BeadSlot) -> bool {
    //    unimplemented!()
    //}
    //
    //// checks if dest is a predecessor of src already,
    //// detecting a loop
    //pub fn loop_check(&mut self) {
    //    //self.link_type = StichLink::Feedback { bead: uuid::Uuid::new_v4() }
    //}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum StichLink {
    // right next to each other without caveats
    Adjacent,
    // indicating a connection to a predecessor implying a loop
    Feedback { bead: uuid::Uuid },
}
