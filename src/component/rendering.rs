use crate::component::Component;
use ggez::Context;
use ggez::graphics;
use ggez::nalgebra as na;

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
