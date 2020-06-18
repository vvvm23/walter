use std::sync::{Arc, Mutex, RwLock};
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
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
    pub dx: f32,
    pub dy: f32,
}

impl VelocityComponent {
    pub fn new(dx: f32, dy: f32) -> VelocityComponent {
        VelocityComponent {
            dx: dx,
            dy: dy,
        }
    }
}

