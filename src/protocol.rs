use schemars::JsonSchema;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema, Clone)]
pub struct BoardMessage {
    pub actions: Vec<BoardAction>,
}

impl BoardMessage {
    pub fn new(id: u8) -> Self {
        Self {
            actions: vec![BoardAction::StartBlock, BoardAction::BlockNumber(id)],
        }
    }

    pub fn push(&mut self, action: BoardAction) {
        self.actions.push(action);
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut result = vec![];

        for action in &self.actions {
            result.extend_from_slice(&action.serialize());
        }

        result
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema, Clone)]
pub enum BoardAction {
    StartBlock,
    BlockNumber(u8),
    StartDrawing,
    StopDrawing,
    PenUp,
    PenDown,
    Eraser,
    Wait(u8),
    Move(u16, u16),
}

impl BoardAction {
    pub fn serialize(&self) -> [u8; 3] {
        match self {
            BoardAction::StartBlock => packet(4009, 4001),
            BoardAction::BlockNumber(n) => packet(4009, *n as u16),
            BoardAction::StartDrawing => packet(4001, 4001),
            BoardAction::StopDrawing => packet(4002, 0000),
            BoardAction::PenUp => packet(4003, 0000),
            BoardAction::PenDown => packet(4004, 0000),
            BoardAction::Eraser => packet(4005, 0000),
            BoardAction::Wait(sec) => packet(4006, *sec as u16),
            BoardAction::Move(x, y) => packet(*x, *y),
        }
    }
}

fn packet(c1: u16, c2: u16) -> [u8; 3] {
    // Combine C1 and C2 into a 24-bit number
    let num: u32 = ((c1 as u32) << 12) | (c2 as u32);

    // Convert the 24-bit number into bytes
    let byte1 = ((num >> 16) & 0xFF) as u8;
    let byte2 = ((num >> 8) & 0xFF) as u8;
    let byte3 = (num & 0xFF) as u8;

    [byte1, byte2, byte3]
}
