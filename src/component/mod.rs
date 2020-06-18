pub mod physics;
pub mod rendering;

pub enum Component {
    PositionComponent(physics::PositionComponent),
    VelocityComponent(physics::VelocityComponent),
    PrimitiveRenderableComponent(rendering::PrimitiveRenderableComponent),
}
