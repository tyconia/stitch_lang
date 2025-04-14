use crate::prelude::*;
use serde::*;

// TODO: bevy feats
#[cfg(feature = "bevy")]
use bevy::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Slot {
    pub title: String,
    pub description: String,
    pub status: FieldStatus,
    pub expected_type: SlotParam,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum FieldStatus {
    Disabled,
    ReadOnly,
    Write,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SlotParam {
    Bool,
    String,
    Number,
    Float,
    Map(HashMap<String, Self>),
    Choice { options: HashSet<String> },
    Stateful(Box<Self>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SlotArg {
    Bool(bool),
    String(String),
    Number(i32),
    Float(f32),
    Map(HashMap<String, Self>),
    Choice {
        selected: Option<String>,
        options: HashSet<String>,
    },
    Stateful(Box<Self>),
}

impl From<SlotArg> for SlotParam {
    fn from(arg: SlotArg) -> Self {
        match arg {
            SlotArg::Bool(_) => SlotParam::Bool,
            SlotArg::String(_) => SlotParam::String,
            SlotArg::Number(_) => SlotParam::Number,
            SlotArg::Float(_) => SlotParam::Float,
            SlotArg::Map(map) => {
                SlotParam::Map(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            SlotArg::Choice { options, .. } => SlotParam::Choice { options },
            SlotArg::Stateful(state) => SlotParam::Stateful(Box::new((*state).into())),
        }
    }
}

impl SlotArg {
    pub fn number(&self) -> Result<i32, RuntimeError> {
        if let &Self::Number(n) = self {
            Ok(n)
        } else {
            Err(RuntimeError::IncompatibleParams {
                expected: "Number".into(),
                received: "Something else".into(),
            })
        }
    }
}
