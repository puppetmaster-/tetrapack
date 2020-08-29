use tetra::{
    graphics::{self, DrawParams,Color},
    math::Vec2,
    Context, Event,
};
use tetra::graphics::{Texture, Rectangle};
use crate::input_action;
use crate::scenes::{Scene, Transition};
use keyframe::Keyframe;
use keyframe::functions::{EaseInOutQuint, EaseInOut, Linear};
use crate::utils::timer::Timer;
use crate::animation::Tween;


#[allow(dead_code)]
pub struct SplashScreenScene {
    atlas: Texture,
    timer: Timer,
    animations: Vec<Tween>,
}

impl SplashScreenScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<SplashScreenScene> {
        let timer = Timer::new_sec(5);
        let tween1 = Tween::from_keyframes(vec![
            Keyframe::new(0.0,0.0,EaseInOutQuint),
            Keyframe::new(5.0,0.5,EaseInOutQuint),
            Keyframe::new(0.0,1.0,EaseInOut)], 0,4,true);
        let tween2 = Tween::from_keyframes(vec![
            Keyframe::new(-0.1,0.0,EaseInOutQuint),
            Keyframe::new(0.1,0.5,EaseInOutQuint),
            Keyframe::new(-0.1,1.0,EaseInOut)],0,4,true);
        let tween3 = Tween::from_keyframes(vec![
            Keyframe::new(0.0,0.0,Linear),
            Keyframe::new(6.283_185_5,1.0,Linear)],0,10,true);
        let tween = vec![tween1, tween2, tween3];
        Ok(SplashScreenScene {
            atlas: Texture::from_file_data(ctx, include_bytes!("../../resources/splashScreen/atlas.png"))?,
            timer,
            animations: tween,
        })
    }
}

impl Scene for SplashScreenScene {
    fn init(&mut self)-> tetra::Result{
        Ok(())
    }

    fn save(&mut self)-> tetra::Result{
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> tetra::Result<Transition> {
        self.animations[0].update();
        self.animations[1].update();
        self.animations[2].update();
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        let logo_pos = Vec2::new(120.0, 0.0);
        let gear_pos = logo_pos + Vec2::new(256.0, 256.0);
        let text_pos = logo_pos + Vec2::new(310.0, 250.0);
        let alien_pos = logo_pos + Vec2::new(500.0, 224.0) + Vec2::new(self.animations[0].value(), self.animations[0].value()*2.5);
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: alien_pos,
            clip: get_clip("tetra_alien_body"),
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: alien_pos+Vec2::new(0.0,12.0),
            clip: get_clip("tetra_alien_eye1"),
            origin: Vec2::new(0.0,48.0),
            rotation: self.animations[1].value()*-1.0,
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: text_pos+Vec2::new(3.0,-32.0),
            clip: get_clip("tetra_text1"),
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: text_pos+Vec2::new(-8.0,20.0),
            clip: get_clip("tetra_text2"),
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: alien_pos+Vec2::new(10.0,30.0),
            clip: get_clip("tetra_alien_eye2"),
            origin: Vec2::new(0.0,26.0),
            rotation: self.animations[1].value(),
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: gear_pos,
            clip: get_clip("tetra_gear_wheel"),
            origin: get_origin("tetra_gear_wheel").unwrap_or_default(),
            rotation: self.animations[2].value(),
            ..Default::default()
        });
        graphics::draw(ctx, &self.atlas, DrawParams{
            position: gear_pos,
            clip: get_clip("tetra_symbol"),
            origin: get_origin("tetra_symbol").unwrap_or_default(),
            ..Default::default()
        });
        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result<Transition> {
        if self.timer.finished() || input_action::is_any_key(&event){
            Ok(Transition::Pop)
        }else{
            Ok(Transition::None)
        }
    }
}

fn get_clip(name: &str) -> Option<Rectangle>{
    match name{
        "tetra_symbol" => Some(Rectangle::new(282.0,0.0,124.0,142.0)),
        "tetra_text1" => Some(Rectangle::new(282.0,144.0,210.0,54.0)),
        "tetra_text2" => Some(Rectangle::new(0.0,184.0,266.0,40.0)),
        "tetra_alien_body" => Some(Rectangle::new(0.0,0.0,20.0,24.0)),
        "tetra_alien_eye2" => Some(Rectangle::new(48.0,0.0,48.0,26.0)),
        "tetra_alien_eye1" => Some(Rectangle::new(22.0,0.0,24.0,48.0)),
        "tetra_gear_wheel" => Some(Rectangle::new(98.0,0.0,182.0,182.0)),
        _ => None
    }
}

fn get_origin(name: &str) -> Option<Vec2<f32>>{
    match name {
        "tetra_symbol" => Some(Vec2::new(62.0, 71.0)),
        "tetra_text1" => Some(Vec2::new(105.0, 27.0)),
        "tetra_text2" => Some(Vec2::new(133.0, 20.0)),
        "tetra_alien_body" => Some(Vec2::new(10.0, 12.0)),
        "tetra_alien_eye2" => Some(Vec2::new(24.0, 13.0)),
        "tetra_alien_eye1" => Some(Vec2::new(12.0, 24.0)),
        "tetra_gear_wheel" => Some(Vec2::new(91.0, 91.0)),
        _ => None
    }
}
