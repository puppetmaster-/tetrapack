pub mod keyframe;

use crate::utils::timer::Timer;
use self::keyframe::{AnimationSequence, Keyframe};

pub struct Tween {
	timer: Timer,
	sequence: Option<AnimationSequence<f32>>,
	repeat: bool,
}

impl Tween{
	pub fn from_keyframes(keyframes: Vec<Keyframe<f32>>,duration: u64,repeat: bool) -> Tween{
		let sequence = AnimationSequence::from(keyframes);
		Self{
			timer: Timer::new_sec(duration),
			sequence: Some(sequence),
			repeat,
		}
	}

	pub fn update(&mut self){
		if let Some(s) = self.sequence.as_mut() {
			s.advance_to(self.timer.value() as f64);
		}
		if self.timer.finished() && self.repeat{
			self.timer.restart();
		}
	}

	pub fn value(&self) -> f32{
		if let Some(s) = self.sequence.as_ref(){
			s.now()
		}else{
			1.0
		}
	}
}