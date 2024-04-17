pub mod coords;
pub mod text;

#[derive(Clone, Copy, PartialEq)]
pub enum SBA {
    PenDown,
    PenUp,
    Eraser,
    Move(f32, f32),
}
