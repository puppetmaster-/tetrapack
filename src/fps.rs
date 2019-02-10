
use std::collections::VecDeque;
use std::time::Instant;

use tetra::graphics::{Drawable, DrawParams, Font, Text, Color};
use tetra::{Context};

use tetra::time;

pub struct Fps{
    color: Color,
    text: Text,
    too_low: bool,
    too_low_color: Color,
}

impl Fps{
    pub fn new<P>(font: Font, params: P) -> Fps
        where
        P: Into<FpsParams>,
        {
            let params = params.into();

            let text = Text::new("FPS", font, params.size, );
            let color = params.color;
            let too_low_color = Color::rgb(1.0, 0.0, 0.0);
            Fps {
                color,
                text,
                too_low_color,
                too_low: false,
            }
        }

    pub fn update(&mut self, ctx: &mut Context){
        let fps = time::get_fps(ctx) as i64;

        if fps < 60 && !self.too_low {
            self.too_low = true
        }else if fps >= 60 && self.too_low {
            self.too_low = false
        }

        self.text.set_content(format!("FPS {:?}",fps));
    }

    pub fn black(&mut self) -> &mut Self{
        self.color(Color::rgb(0.0,0.0,0.0))
    }

    pub fn white(&mut self) -> &mut Self{
        self.color(Color::rgb(1.0,1.0,1.0))
    }

    pub fn color(&mut self, color: Color) -> &mut Self{
        self.color = color;
        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self{
        self.text.set_size(size);
        self
    }
}

impl Drawable for Fps {
    fn draw<P>(&self, ctx: &mut Context, params: P)
        where
            P: Into<DrawParams>,
    {
        let mut params = params.into();
        
        if self.too_low{
            params.color = self.too_low_color;
        }else{
            params.color = self.color;
        }

        self.text.draw(ctx, params)}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FpsParams {
    color: Color,
    size: f32,
}
impl FpsParams {
    /// Creates a new set of `FpsParams`.
    pub fn new() -> FpsParams {
        FpsParams::default()
    }

    /// Sets the color.
    pub fn color(mut self, color: Color) -> FpsParams {
        self.color = color;
        self
    }
}

impl Default for FpsParams {
    fn default() -> FpsParams {
        FpsParams {
            color: Color::WHITE,
            size: 12.0,
        }
    }
}

impl From<Color> for FpsParams {
    fn from(color: Color) -> FpsParams {
        FpsParams {
            color,
            ..FpsParams::default()
        }
    }
}

impl From<f32> for FpsParams {
    fn from(size: f32) -> FpsParams {
        FpsParams {
            size,
            ..FpsParams::default()
        }
    }
}