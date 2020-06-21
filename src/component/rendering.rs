use crate::component::Component;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use mint;
use std::sync::Arc;
use std::collections::VecDeque;

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
    pub texture: Arc<graphics::Image>,
    pub scale: mint::Vector2<f32>,
}

impl SpriteRenderableComponent {
    pub fn new(texture: Arc<graphics::Image>, scale_x: f32, scale_y: f32) -> Component {
        Component::SpriteRenderableComponent (
            SpriteRenderableComponent {
                texture: Arc::clone(&texture),
                scale: mint::Vector2 {x: scale_x, y: scale_y},
            }
        )
    }
}

pub struct BackgroundComponent {
    pub texture: Arc<graphics::Image>,
    pub scale: mint::Vector2<f32>,
}

impl BackgroundComponent {
    pub fn new(texture: Arc<graphics::Image>) -> Component {
        Component::BackgroundComponent (
            BackgroundComponent {
                texture: Arc::clone(&texture),
                scale: mint::Vector2 {x: 1600.0 / texture.width() as f32, y: 1200.0 / texture.height() as f32 },
            }
        )
    }
}

pub struct TextBoxComponent {
    xs: f32,    ys: f32,
    p_colour: graphics::Color,
    s_colour: graphics::Color,

    lines: VecDeque<String>,
    nb_lines: u8,
    capacity: u8,
}

impl TextBoxComponent {
    pub fn new(capacity: u8, xs: f32, ys: f32, pc: graphics::Color, sc: graphics::Color) -> Component {
        Component::TextBoxComponent(
            TextBoxComponent {
                xs: xs,
                ys: ys,
                p_colour: pc,
                s_colour: sc,

                lines: VecDeque::new(),
                nb_lines: 0,
                capacity: capacity,
            }
        )
    }

    pub fn draw(&self, x: f32, y: f32, ctx: &mut Context) -> GameResult {
        const BORDER_SIZE: f32 = 3.0;
        const LINE_SPACE: f32 = 20.0;
        let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {x: 0.0, y: 0.0, w: self.xs, h: self.ys},
            self.s_colour,
        ).unwrap();
        let draw_param = graphics::DrawParam::default()
            .dest(na::Point2::new(x, y));
        graphics::draw(ctx, &mesh, draw_param)?;

        let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {x: BORDER_SIZE, y: BORDER_SIZE, w: self.xs - 2.0*BORDER_SIZE, h: self.ys - 2.0*BORDER_SIZE},
            self.p_colour,
        ).unwrap();

        let draw_param = graphics::DrawParam::default()
            .dest(na::Point2::new(x, y));
        graphics::draw(ctx, &mesh, draw_param)?;

        for (i, l) in self.lines.iter().enumerate() {
            let line_text = graphics::Text::new(format!("{}", l));
            let point = na::Point2::new(
                x + BORDER_SIZE,
                y + BORDER_SIZE + LINE_SPACE * i as f32
            );
            let draw_param = graphics::DrawParam::default()
                .dest(point);
            graphics::draw(ctx, &line_text, draw_param)?;
        }
        
        Ok(())
    }

    pub fn add_line(&mut self, line: &str) {
        self.lines.push_back(line.to_string());
        self.nb_lines += 1;
        if self.nb_lines + 1 > self.capacity {
            self.lines.pop_front();
            self.nb_lines -= 1;
        }
    }

}
