use std::sync::{Arc, Mutex, RwLock};
use crate::component::Component;

#[derive(Debug)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Component {
        Component::PositionComponent(
            PositionComponent {
                x: x,
                y: y,
            }
        )
    }
}

pub struct VelocityComponent {
    pub dx: f32,
    pub dy: f32,
}

impl VelocityComponent {
    pub fn new(dx: f32, dy: f32) -> Component {
        Component::VelocityComponent(
            VelocityComponent {
                dx: dx,
                dy: dy,
            }
        )
    }
}

