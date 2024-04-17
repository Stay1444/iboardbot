use cgmath::Vector2;

use crate::protocol::{BoardAction, BoardMessage};

use super::{coords::CoordinateProjector, SBA};

struct TextProjector;

pub fn write(
    msg: &mut BoardMessage,
    lines: Vec<String>,
    scale: f32,
    proyector: CoordinateProjector,
) {
    let mut actions = vec![];
    for line in lines {
        for char in line.chars() {
            match char {
                'T' => {
                    actions.extend_from_slice(&[
                        SBA::Move(0.1, 0.0),
                        SBA::PenDown,
                        SBA::Move(0.9, 0.0),
                        SBA::PenUp,
                        SBA::Move(0.45, 0.0),
                        SBA::PenDown,
                        SBA::Move(0.45, 1.0),
                    ]);
                }
                _ => todo!(),
            }
        }
    }
}
