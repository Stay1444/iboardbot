use bevy_math::Rect;

pub struct CoordinateProjector(Rect);

impl CoordinateProjector {
    pub fn new(rect: Rect) -> Self {
        Self(rect)
    }
}
