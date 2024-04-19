use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::protocol::BoardMessage;

#[derive(Serialize, JsonSchema, Clone)]
pub struct Board {
    pub id: String,
    pub state: BoardState,
    pub last_update: DateTime<Utc>,
    pub details: BoardDetails,

    pub available: (f32, f32, f32, f32),
}

#[derive(Serialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoardState {
    Unknown,
    Disconnected,
    Ready,
    Working(Job),
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
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

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct BoardDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub struct Job {
    pub id: u32,
    pub action: JobAction,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
pub enum JobAction {
    DrawSVG(SVGSource),
    DrawSVGGroup(Vec<SVGSource>),
    Calibrate,
    WriteText(WriteText),
    Raw(BoardMessage),
    Erase,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
pub enum SVGSource {
    Raw(String),
    Url(String),
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
pub struct WriteText {
    pub text: String,
    #[serde(default)]
    pub font: Option<String>,
}
