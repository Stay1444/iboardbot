use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema)]
pub struct Board {
    pub id: String,
    pub state: BoardState,
    pub details: BoardDetails,
}

#[derive(Serialize, JsonSchema, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoardState {
    Disconnected,
    Ready,
    Working(Job),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct BoardDetails {
    pub dimensions: BoardDimensions,
}

impl Default for BoardDetails {
    fn default() -> Self {
        Self {
            dimensions: BoardDimensions {
                width: 1000,
                height: 1000,
            },
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct BoardDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Eq, Clone)]
pub struct Job {
    pub id: u32,
    pub action: JobAction,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, PartialEq, Eq)]
pub enum JobAction {
    Write(Vec<String>),
}
