use tetra::{Context, Event, window, State, graphics};
use crate::scenes::splash_screen::SplashScreenScene;
use tetra::graphics::scaling::ScreenScaler;
use crate::TetraVec2;

pub mod splash_screen;

pub trait Scene {
	fn init(&mut self) -> tetra::Result;
	fn save(&mut self) -> tetra::Result;
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result;
	fn event(&mut self, ctx: &mut Context,event: Event) -> tetra::Result<Transition>;
}

#[allow(dead_code)]
pub enum Transition<'a> {
	None,
	Push(Box<dyn Scene>),
	Load(&'a str),
	Pop,
	Quit,
}

pub struct SceneManager {
	scenes: Vec<Box<dyn Scene>>,
	screen_scaler: Option<ScreenScaler>,
}

#[allow(dead_code)]
impl SceneManager {
	pub fn new(initial_scene: Box<dyn Scene>) -> SceneManager {
		SceneManager {
			scenes: vec![initial_scene],
			screen_scaler: None
		}
	}
	pub fn new_with_splash_screen(ctx: &mut Context,initial_scene: Box<dyn Scene>) -> SceneManager {
		SceneManager {
			scenes: vec![initial_scene, Box::new(SplashScreenScene::new(ctx).unwrap())],
			screen_scaler: None
		}
	}

	pub fn set_screen_scaler(mut self,screen_scaler: ScreenScaler) -> Self{
		self.screen_scaler = Option::from(screen_scaler);
		self
	}

	fn init_scene(&mut self) -> tetra::Result{
		if let Some(active_scene) = self.scenes.last_mut() {
			active_scene.init()?
		}
		Ok(())
	}
}

impl State for SceneManager {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		match self.scenes.last_mut() {
			Some(active_scene) => match active_scene.update(ctx)? {
				Transition::None => {}
				Transition::Load(s) =>{
					println!("{}",s);
				}
				Transition::Push(s) => {
					active_scene.save();
					self.scenes.push(s);
					self.init_scene()?;
				}
				Transition::Pop => {
					active_scene.save();
					self.scenes.pop();
					self.init_scene()?;
				}
				Transition::Quit => {
					window::quit(ctx)
				}
			},
			None => window::quit(ctx),
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		match self.screen_scaler.as_ref(){
			Some(scaler) => graphics::set_canvas(ctx, scaler.canvas()),
			_ => {}
		}

		if let Some(active_scene) = self.scenes.last_mut() {
			active_scene.draw(ctx)?
		}

		match self.screen_scaler.as_ref(){
			Some(scaler) => {
				graphics::reset_canvas(ctx);
				graphics::reset_transform_matrix(ctx);
				graphics::draw(ctx, scaler, TetraVec2::zero());
			},
			_ => {}
		}
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
		if let Event::Resized { width, height } = event {
			match self.screen_scaler.as_mut(){
				Some(scaler) => scaler.set_outer_size(width, height),
				_ => {}
			}
		}
		match self.scenes.last_mut() {
			Some(active_scene) => match active_scene.event(ctx, event)?{
				Transition::None => {}
				Transition::Load(s) =>{
					println!("{}",s);
				}
				Transition::Push(s) => {
					self.scenes.push(s);
					self.init_scene()?;
				}
				Transition::Pop => {
					self.scenes.pop();
					self.init_scene()?;
				}
				Transition::Quit => {
					window::quit(ctx)
				}
			},
			None => window::quit(ctx),
		}
		Ok(())
	}
}


