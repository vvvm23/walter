pub struct PositionComponent {
    x: f32,
    y: f32,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> PositionComponent {
        PositionComponent {
            x: x,
            y: y,
        }
    }
}

pub struct VelocityComponent {
    dx: f32,
    dy: f32,
}

impl VelocityComponent {
    pub fn new(dx: f32, dy: f32) -> VelocityComponent {
        VelocityComponent {
            dx: dx,
            dy: dy,
        }
    }
}

