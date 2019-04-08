use tetra::graphics::{Drawable, DrawParams, Vec2, Rectangle, Texture, Color, Text, Font, NineSlice};
use tetra::input::{self, MouseButton};
use tetra::{Context, glm};
use std::collections::hash_map::HashMap;

#[allow(dead_code)]
pub struct Button{
	pressed: bool,
	visible: bool,
	disabled: bool,
	position: Vec2,
	state: ButtonState,
	text: Text,
	text_frame_size: f32,
	text_colors: HashMap<ButtonState, Color>,
	textures: HashMap<ButtonState, Texture>,
	panel: NineSlice,
}

impl Button{
	pub fn new(ctx: &mut Context, position: Vec2) -> tetra::Result<Button> {
		let width = 50.0;
		let height = 20.0;
		let textures = get_textures(ctx)?;
		let panel = NineSlice::new(textures[&ButtonState::Normal].clone(),width,height,Rectangle::new(5.0,5.0,6.0,6.0));

		Ok(Button{
			textures,
			position,
			pressed: false,
			visible: true,
			disabled: false,
			state: ButtonState::Normal,
			text: Text::new("Ög", Font::default(), 18.0),
			text_colors: get_text_colors(),
			text_frame_size: 10.0,
			panel,
		})
	}

	pub fn text(mut self, text: &str) -> Self{
		self.text.set_content(text);
		self
	}

	pub fn set_text(&mut self, text: &str){
		self.text.set_content(text);
	}

	pub fn position(mut self, position: Vec2) -> Self{
		self.position = position;
		self
	}

	pub fn set_position(&mut self, position: Vec2){
		self.position = position;
	}

	pub fn visible(mut self, visible: bool) -> Self{
		self.visible = visible;
		self
	}

	pub fn text_color(mut self, state: ButtonState, color: Color) ->Self{
		self.text_colors.entry(state).or_insert(color);
		self
	}

	pub fn texture(mut self, state: ButtonState, texture: Texture) -> Self{
		self.textures.entry(state).or_insert(texture);
		self
	}

	//not the best way to do it
	pub fn is_pressed(&self) -> bool{
		if self.pressed{
			return true;
		}
		false
	}

	//not the best way to do it
	pub fn get_pressed(&mut self) -> bool{
		if self.pressed{
			self.pressed = false;
			self.state = ButtonState::Normal;
			return true;
		}
		false
	}

	pub fn update(&mut self, ctx: &mut Context){
		if self.visible && !self.disabled{
			let mouse_position = glm::round(&input::get_mouse_position(ctx));

			let text_bounds = self.text.get_bounds(ctx).unwrap();
			if self.panel.width() < text_bounds.width+self.text_frame_size{
				self.panel.set_width(text_bounds.width+self.text_frame_size);
			}
			if self.panel.height() < text_bounds.height+self.text_frame_size{
				self.panel.set_height(text_bounds.height+self.text_frame_size);
			}

			let bounds = Rectangle::new(0.0,0.0, self.panel.width(), self.panel.height());

			if is_inside_hover_area(self.position, bounds, mouse_position) {
				if input::is_mouse_button_down(ctx, MouseButton::Left){
					self.state = ButtonState::Pressed;
				}else{
					self.state = ButtonState::Hover;
					self.text.set_content("OK");
				}
				if input::is_mouse_button_released(ctx, MouseButton::Left){
					self.pressed = true;
				}
			}else{
				self.state = ButtonState::Normal;
				self.text.set_content("Ög");
			}
			self.panel.set_texture(self.textures[&self.state].clone());
		}
	}
}

impl Drawable for Button {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		if self.visible{
			let mut params = params.into();

			params.position = self.position;

			self.panel.draw(ctx, self.position);
			let bounds = self.text.get_bounds(ctx).unwrap();
			self.text.draw(ctx, DrawParams::new()
				.color(self.text_colors[&self.state])
				.position(Vec2::new(self.position.x + self.panel.width() / 2.0, self.position.y + self.panel.height() / 2.0))
				.origin(Vec2::new(bounds.width / 2.0, bounds.height / 2.0))
			);
		}
	}
}

fn is_inside_hover_area(draw_position: Vec2, area: Rectangle, position: Vec2) -> bool{
	let pos_x = draw_position.x;
	let pos_y = draw_position.y;
	!(position.x < area.x + pos_x ||
		position.y < area.y + pos_y ||
		position.x > area.x + pos_x + area.width ||
		position.y > area.y + pos_y + area.height
	)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonState {
	Normal,
	Hover,
	Pressed,
	Disabled,
}

fn get_textures(ctx: &mut Context) -> tetra::Result<HashMap<ButtonState, Texture>>{
	let mut textures: HashMap<ButtonState, Texture> = HashMap::new();
	textures.insert(ButtonState::Normal, Texture::from_file_data(ctx, include_bytes!("../../resources/button_0.png"))?);
	textures.insert(ButtonState::Hover, Texture::from_file_data(ctx, include_bytes!("../../resources/button_1.png"))?);
	textures.insert(ButtonState::Pressed, Texture::from_file_data(ctx, include_bytes!("../../resources/button_2.png"))?);
	textures.insert(ButtonState::Disabled, Texture::from_file_data(ctx, include_bytes!("../../resources/button_3.png"))?);
	Ok(textures)
}

fn get_text_colors() -> HashMap<ButtonState, Color>{
	let mut text_colors: HashMap<ButtonState, Color> = HashMap::new();
	text_colors.insert(ButtonState::Normal, Color::rgb(1.0,1.0,1.0));
	text_colors.insert(ButtonState::Hover, Color::rgb(0.984,0.875,0.42));
	text_colors.insert(ButtonState::Pressed, Color::rgb(0.075,0.698,0.949));
	text_colors.insert(ButtonState::Disabled, Color::rgb(0.2,0.2,0.2));
	text_colors
}