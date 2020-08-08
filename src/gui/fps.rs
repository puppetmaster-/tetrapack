
use tetra::graphics::{Drawable, DrawParams, Color};
use tetra::{Context};

use tetra::time;
use tetra::graphics::text::{Text, Font};

pub struct Fps{
    text: Text,
    is_low: bool,
    color: Color,
    alert_color: Color,
    update_cycle: f32,
    update_periode: f32,
    update_frequence: f32,
}

#[allow(dead_code)]
impl Fps{
    pub fn new<P>(font: Font, params: P) -> Fps
        where
            P: Into<FpsParams>,
    {
        let params = params.into();
        let text = Text::new("FPS", font);

        Fps {
            text,
            color: params.color,
            alert_color: params.alert_color,
            update_cycle: params.update_periode,
            update_periode: params.update_periode,
            is_low: false,
            update_frequence: 0.1,
        }
    }

    pub fn update(&mut self, ctx: &mut Context){
        self.update_cycle += self.update_frequence;

        if self.update_cycle >= self.update_periode {
            let fps = time::get_fps(ctx) as i64;

            if fps < 60 && !self.is_low {
                self.is_low = true
            } else if fps >= 60 && self.is_low {
                self.is_low = false
            }

            self.text.set_content(format!("FPS {:?}", fps));
            self.update_cycle = 0.0;
        }
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

    /*
    pub fn font_size(&mut self, size: f32) -> &mut Self{
        self.text.set_size(size);
        self
    }*/
}

impl Drawable for Fps {
    fn draw<P>(&self, ctx: &mut Context, params: P)
        where
            P: Into<DrawParams>,
    {
        let mut params = params.into();

        if self.is_low {
            params.color = self.alert_color;
        }else{
            params.color = self.color;
        }

        self.text.draw(ctx, params)}
}

#[derive(Debug, Clone, PartialEq)]
pub struct FpsParams {
    color: Color,
    alert_color: Color,
    font_size: f32,
    update_periode: f32,
}

#[allow(dead_code)]
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

    /// Sets the alarm color.
    pub fn alert_color(mut self, color: Color) -> FpsParams {
        self.alert_color = color;
        self
    }

    /// Sets the size.
    pub fn font_size(mut self, size: f32) -> FpsParams {
        self.font_size = size;
        self
    }

    pub fn update_periode(mut self, update_periode: f32) -> FpsParams {
        self.update_periode = update_periode;
        self
    }
}

impl Default for FpsParams {
    fn default() -> FpsParams {
        FpsParams {
            font_size: 12.0,
            update_periode: 1.0,
            color: Color::rgb(1.0,1.0,1.0),
            alert_color: Color::rgb(1.0, 0.0, 0.0),
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
            font_size: size,
            ..FpsParams::default()
        }
    }
}