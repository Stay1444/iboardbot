use crate::protocol::BoardAction;

pub mod coords;
pub mod text;

#[derive(Clone, PartialEq, Debug)]
pub enum SBA {
    PenDown,
    PenUp,
    Eraser,
    Move(f32, f32),
}

impl Into<BoardAction> for SBA {
    fn into(self) -> BoardAction {
        match self {
            SBA::PenDown => BoardAction::PenDown,
            SBA::PenUp => BoardAction::PenUp,
            SBA::Eraser => BoardAction::Eraser,
            SBA::Move(x, y) => BoardAction::Move(x as u16, y as u16),
        }
    }
}
