use bevy_math::{Rect, Vec2};

use crate::protocol::{BoardAction, BoardMessage};

use super::{coords::CoordinateProjector, SBA};

struct TextProjector {
    projector: CoordinateProjector,
    scale: f32,
    next_target: Rect,

    instructions: Vec<SBA>,
    erase: bool,
}

impl TextProjector {
    fn new(projector: CoordinateProjector, scale: f32, erase: bool) -> Self {
        Self {
            projector,
            scale,
            next_target: Rect::from_corners(Vec2::ZERO, Vec2::new(scale, scale)),
            instructions: vec![],
            erase,
        }
    }

    fn project(&mut self, mut char_inst: Vec<SBA>) {
        dbg!(&self.next_target);
        if let Some(last) = char_inst.last() {
            if !matches!(last, SBA::PenUp) {
                char_inst.push(SBA::PenUp);
            }
        };

        if self.erase {
            char_inst = char_inst
                .into_iter()
                .filter(|x| matches!(x, SBA::Move(_, _)))
                .collect();
        }

        for inst in &mut char_inst {
            if let SBA::Move(x, y) = inst {
                *x = x.clamp(0.0, 1.0);
                *y = y.clamp(0.0, 1.0);

                *x = (*x * self.next_target.size().x) + self.next_target.min.x;
                *y = (*y * self.next_target.size().y) + self.next_target.min.y;
            }
        }

        self.instructions.append(&mut char_inst);

        let padding = self.scale / 2.0;
        self.next_target = Rect::from_corners(
            Vec2::new(self.next_target.max.x + padding, self.next_target.min.y),
            self.next_target.max + Vec2::new(self.scale, 0.0) + Vec2::new(padding, 0.0),
        );
    }

    fn new_line(&mut self) {
        let padding = Vec2::new(0.0, self.scale / 2.0);
        self.next_target = Rect::from_corners(
            Vec2::new(0.0, self.scale) + padding,
            Vec2::new(self.scale, self.scale) + padding,
        );
    }

    fn write(mut self, message: &mut BoardMessage) {
        if self.erase {
            self.instructions.push(SBA::PenUp);
            self.instructions.insert(0, SBA::Eraser);
        }
        for mut inst in self.instructions {
            if let SBA::Move(_, y) = &mut inst {
                if self.erase {
                    *y += 150.0;
                }
            }
            message.push(inst.into());
        }
    }
}

pub fn write(
    msg: &mut BoardMessage,
    lines: Vec<String>,
    scale: f32,
    projector: CoordinateProjector,
    erase: bool,
) {
    let mut projector = TextProjector::new(projector, scale, erase);
    for line in lines {
        for char in line.chars() {
            let actions = get_char(char);
            projector.project(actions);
        }
        projector.new_line();
    }

    projector.write(msg);
}

fn get_char(char: char) -> Vec<SBA> {
    let mut actions = vec![];
    actions.extend_from_slice(match char {
        'A' => &[
            SBA::Move(0.5, 0.0),
            SBA::PenDown,
            SBA::Move(0.0, 1.0),
            SBA::PenUp,
            SBA::Move(0.5, 0.0),
            SBA::PenDown,
            SBA::Move(1.0, 1.0),
            SBA::PenUp,
            SBA::Move(0.2, 0.5),
            SBA::PenDown,
            SBA::Move(0.8, 0.5),
        ],
        'B' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.8, 0.0),
            SBA::Move(0.8, 0.4),
            SBA::Move(0.0, 0.4),
            SBA::Move(0.0, 1.0),
            SBA::Move(1.0, 1.0),
            SBA::Move(1.0, 0.4),
            SBA::Move(0.0, 0.4),
            SBA::Move(0.0, 0.0),
        ],
        'C' => &[
            SBA::Move(1.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.0),
            SBA::Move(0.0, 1.0),
            SBA::Move(1.0, 1.0),
        ],
        'D' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.8, 0.0),
            SBA::Move(1.0, 0.2),
            SBA::Move(1.0, 0.8),
            SBA::Move(0.8, 1.0),
            SBA::Move(0.0, 1.0),
            SBA::Move(0.0, 0.0),
        ],
        'E' => &[
            SBA::Move(0.0, 1.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.0),
            SBA::Move(0.0, 1.0),
            SBA::Move(1.0, 1.0),
            SBA::PenUp,
            SBA::Move(0.8, 0.5),
            SBA::PenDown,
            SBA::Move(0.0, 0.0),
        ],
        'F' => &[
            SBA::Move(0.0, 0.0),
            SBA::Move(1.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.0),
            SBA::Move(0.0, 1.0),
            SBA::PenUp,
            SBA::Move(0.0, 0.7),
            SBA::PenDown,
            SBA::Move(1.0, 0.7),
        ],
        'H' => &[
            SBA::Move(0.0, 1.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.5),
            SBA::Move(1.0, 0.5),
            SBA::Move(1.0, 0.0),
            SBA::PenUp,
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.5),
            SBA::PenUp,
            SBA::Move(1.0, 0.5),
            SBA::PenDown,
            SBA::Move(1.0, 1.0),
        ],
        'I' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(1.0, 0.0),
            SBA::PenUp,
            SBA::Move(1.0, 1.0),
            SBA::PenDown,
            SBA::Move(0.0, 1.0),
            SBA::PenUp,
            SBA::Move(0.5, 0.0),
            SBA::PenDown,
            SBA::Move(0.5, 1.0),
        ],
        'L' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(0.0, 1.0),
            SBA::Move(1.0, 1.0),
        ],
        'N' => &[
            SBA::Move(0.0, 1.0),
            SBA::PenDown,
            SBA::Move(0.0, 0.0),
            SBA::Move(1.0, 1.0),
            SBA::Move(1.0, 0.0),
        ],
        'O' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(1.0, 0.0),
            SBA::Move(1.0, 1.0),
            SBA::Move(0.0, 1.0),
            SBA::Move(0.0, 0.0),
        ],
        'T' => &[
            SBA::Move(0.0, 0.0),
            SBA::PenDown,
            SBA::Move(1.0, 0.0),
            SBA::PenUp,
            SBA::Move(0.5, 0.0),
            SBA::PenDown,
            SBA::Move(0.5, 1.0),
        ],
        ' ' | _ => &[],
    });
    actions
}
