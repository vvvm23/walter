use crate::component::Component;
use ggez::Context;
use ggez::graphics;
use ggez::nalgebra as na;
use mint;

pub enum Shape {
    Circle{ r: f32 }, // Radius
    Rectangle{ w: f32, h: f32 }, // width, height
}

pub struct PrimitiveRenderableComponent {
    shape: Shape,
    draw_mode: graphics::DrawMode,
    colour: graphics::Color,
}

impl PrimitiveRenderableComponent {
    pub fn new(shape: Shape, draw_mode: graphics::DrawMode, colour: graphics::Color) -> Component {
        Component::PrimitiveRenderableComponent(
            PrimitiveRenderableComponent {
                shape: shape,
                draw_mode: draw_mode,
                colour: colour,
            }
        )
    }

    pub fn build_mesh(&self, ctx: &mut Context) -> graphics::Mesh {
        match self.shape {
            Shape::Circle{r} => graphics::Mesh::new_circle(
                ctx,
                self.draw_mode,
                na::Point2::new(0.0, 0.0),
                r,
                1.0,
                self.colour,
            ),
            Shape::Rectangle{w,h} => graphics::Mesh::new_rectangle(
                ctx,
                self.draw_mode,
                graphics::Rect {x: 0.0, y: 0.0, w: w, h: h},
                self.colour
            )
        }.unwrap()
    }
}

pub struct SpriteRenderableComponent {
    pub texture_path: String,
    pub texture: graphics::Image,
    pub scale: mint::Vector2<f32>,
}

impl SpriteRenderableComponent {
    pub fn new(ctx: &mut Context, path: &str, scale_x: f32, scale_y: f32) -> Component {
        Component::SpriteRenderableComponent (
            SpriteRenderableComponent {
                texture_path: path.to_string(),
                texture: graphics::Image::new(ctx, path).unwrap(),
                scale: mint::Vector2 {x: scale_x, y: scale_y},
            }
        )
    }
}

