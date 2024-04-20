use bevy_math::Vec2;

use crate::protocol::{BoardAction, BoardMessage};

#[derive(Clone, Debug)]
pub struct SBM {
    id: u8,
    pub actions: Vec<SBA>,
}

impl SBM {
    pub fn new(id: u8) -> Self {
        Self {
            actions: vec![],
            id,
        }
    }

    pub fn push(&mut self, action: SBA) {
        self.actions.push(action);
    }

    pub fn scale(&mut self, scale: f32) {
        for action in &mut self.actions {
            if let SBA::Move(x, y) = action {
                *x *= scale;
                *y *= scale;
            }
        }
    }

    pub fn bounds(&self) -> Vec2 {
        let mut max = Vec2::ZERO;

        for action in &self.actions {
            if let SBA::Move(x, y) = action {
                if *x > max.x {
                    max = Vec2::new(*x, max.y);
                }

                if *y > max.y {
                    max = Vec2::new(max.x, *y);
                }
            }
        }

        max
    }

    pub fn build(mut self) -> Vec<BoardMessage> {
        let mut pen_down = false;

        let mut messages = vec![];

        while !self.actions.is_empty() {
            let mut msg = BoardMessage::new(self.id + messages.len() as u8);

            if pen_down {
                msg.push(BoardAction::PenDown);
                pen_down = false;
            }

            while msg.len() < 200 && !self.actions.is_empty() {
                let action = self.actions.remove(0);

                if matches!(action, SBA::PenDown) {
                    pen_down = true;
                } else if matches!(action, SBA::PenUp) {
                    pen_down = false;
                }

                msg.push(action.clone().into());
            }

            messages.push(msg);
        }

        if pen_down {
            if let Some(message) = messages.last_mut() {
                message.push(BoardAction::PenUp);
            }
        }

        messages
    }
}

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
