
use std::collections::VecDeque;
use std::time::Instant;

use tetra::graphics::{Drawable, DrawParams, Font, Text};
use tetra::{Context};

use tetra::time;

pub struct Fps{
    fps_tracker: VecDeque<f64>,
    last_frame: Instant,
    text: Text,
}

impl Fps{
    pub fn new(font: Font) -> Fps{
        let text = Text::new("FPS",font,16.0,);
        Fps{
            fps_tracker: VecDeque::new(),
            last_frame: Instant::now(),
            text,
        }
    }
    pub fn update(&mut self){
        let current_frame = Instant::now();
        let elapsed = current_frame - self.last_frame;

        self.fps_tracker.push_back(time::duration_to_f64(elapsed));

        if self.fps_tracker.len() > 200 {
            self.fps_tracker.pop_front();
        }

        let fps = (1.0 / (self.fps_tracker.iter().sum::<f64>() / self.fps_tracker.len() as f64)) as i64 ;

        self.text.set_content(format!("FPS {:?}",fps));

        self.last_frame = current_frame;
    }
}

impl Drawable for Fps {
    fn draw<P>(&self, ctx: &mut Context, params: P)
        where
            P: Into<DrawParams>,
    {
        let params = params.into();
        self.text.draw(ctx, params)}
}
