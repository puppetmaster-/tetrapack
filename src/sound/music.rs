use tetra::audio::{SoundInstance, Sound, SoundState};
use crate::utils::timer::Timer;
use tetra::Context;

pub struct Music{
	repeat_interval: Timer,
	fade_timeframe: Timer,
	music_instance: SoundInstance,
	volume: f32,
	state: MusicState,
	repeat: bool,
}

impl Music {
	pub fn new(ctx: &mut Context, sound_file_data: &[u8], repeat_interval_sec: u64) -> tetra::Result<Self>{
		let sound = Sound::from_file_data(sound_file_data);
		let music_instance = sound.spawn(ctx)?;
		let repeat = repeat_interval_sec > 0;
		Ok(Music{
			repeat_interval: Timer::new_sec(repeat_interval_sec),
			fade_timeframe: Timer::new(1700),
			music_instance,
			volume: 1.0,
			state: MusicState::Waiting,
			repeat
		})
	}

	pub fn volume(&mut self, volume: f32){
		self.volume = volume;
	}

	pub fn repeat_interval(&mut self,repeat_interval: u64){
		self.repeat_interval = Timer::new_sec(repeat_interval);
	}

	pub fn start(&mut self){
		self.music_instance.play();
		self.fade_timeframe.restart();
		self.state = MusicState::FadeIn;
	}

	pub fn update(&mut self){
		match self.state{
			MusicState::Waiting =>{
				if self.repeat && self.repeat_interval.finished(){
					self.start();
				}
			}
			MusicState::FadeIn =>{
				if self.fade_timeframe.finished(){
					self.state = MusicState::Playing;
				}
				self.music_instance.set_volume(self.volume * self.fade_timeframe.value());
			}
			MusicState::Playing =>{
				if let SoundState::Stopped = self.music_instance.state(){
					self.repeat_interval.restart();
					self.state = MusicState::Waiting;
				}
			}
			MusicState::FadeOut =>{
				if self.fade_timeframe.finished(){
					self.state = MusicState::Waiting;
					self.music_instance.stop();
				}
				self.music_instance.set_volume(self.volume * 1.0 - self.fade_timeframe.value());
			}
		}
	}

	pub fn stop(&mut self){
		self.state = MusicState::Waiting;
		self.music_instance.stop()
	}

	pub fn end(&mut self){
		self.state = MusicState::FadeOut;
		self.fade_timeframe.restart()
	}

}

enum MusicState{
	Waiting,
	Playing,
	FadeIn,
	FadeOut,
}
