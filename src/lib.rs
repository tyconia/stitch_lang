mod bead;
mod fabric;
mod runtime;
pub mod stdlib;
mod stitch;
mod types;
pub mod ui;

pub(crate) mod utils;

#[cfg(feature = "bevy")]
mod bevy_feat;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::bead::*;
    pub use super::fabric::*;
    pub use super::runtime::*;
    pub use super::stitch::*;
    pub use super::types::*;
    pub use crate::utils::*;
    pub use uuid::Uuid;

    #[cfg(feature = "bevy")]
    pub use super::bevy_feat::StitchPlugin;

    #[cfg(feature = "bevy")]
    pub use bevy::utils::{HashMap, HashSet};

    #[cfg(not(feature = "bevy"))]
    pub use std::collections::{HashMap, HashSet};
}
