pub mod physics;
pub mod rendering;
pub mod battle;
pub mod audio;

pub enum Component {
    PositionComponent(physics::PositionComponent),
    VelocityComponent(physics::VelocityComponent),
    PrimitiveRenderableComponent(rendering::PrimitiveRenderableComponent),
    SpriteRenderableComponent(rendering::SpriteRenderableComponent),
    BackgroundComponent(rendering::BackgroundComponent),
    FighterComponent(battle::FighterComponent),
}
