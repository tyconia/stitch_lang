use crate::prelude::SlotArg;
use bevy::prelude::*;

#[derive(Debug, Event)]
pub enum RuntimeReception {
    Arg(SlotArg),
    Startup,
    Shutdown,
}
