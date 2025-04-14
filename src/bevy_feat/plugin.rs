use crate::prelude::*;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

pub struct StitchPlugin;

impl Plugin for StitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<super::StitchEvent>()
            .init_resource::<super::StitchRuntime>()
            .add_systems(Startup, (start_runtime,))
            .add_systems(Update, (listen_runtime, poll_runtime));
    }
}



pub fn start_runtime() {}

pub fn listen_runtime(
    mut event: EventReader<super::StitchEvent>,
) {
    for super::StitchEvent(arg) in event.read() {
        
    }
}

pub fn poll_runtime() {}
