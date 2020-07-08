use std::sync::{Arc, Mutex, RwLock};
use crate::component::Component;
use std::f64::consts::PI;

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

pub struct IdleBobComponent {
    pub period: f32,
    pub amplitude: f32,
    pub current_t: f32,
    pub y: f32,
}

impl IdleBobComponent {
    pub fn new(amplitude: f32, period: f32) -> Component {
        Component::IdleBobComponent(
            IdleBobComponent {
                period: period,
                amplitude: amplitude,
                current_t: 0.0,
                y: 0.0,
            }
       )
    }

    pub fn update(&mut self, dt: &f32) {
        self.current_t += dt;
        self.current_t %= self.period;
        self.y = self.amplitude - (self.current_t * PI as f32 / self.period).sin() * self.amplitude;
    }
}
