use tetra::graphics::{Drawable, Color, DrawParams, Vec2, Text, Rectangle, Font};
use tetra::input::{self, MouseButton};
use tetra::{Context, glm};

#[allow(dead_code)]
pub struct TextButton{
	pressed: bool,
	centered: bool,
	visible: bool,
	text: Text,
	color: Color,
	normal_color: Color,
	hover_color: Color,
	pressed_color: Color,
	position: Vec2
}

impl TextButton{
	pub fn new(text: &str, position: Vec2) -> Self {
		let text = Text::new(text, Font::default(), 12.0);
		TextButton{
			pressed: false,
			centered: true,
			visible: true,
			color: Color::rgb(1.0,1.0,1.0),
			normal_color: Color::rgb(1.0,1.0,1.0),
			hover_color: Color::rgb(1.0, 0.0, 0.0),
			pressed_color: Color::rgb(0.0, 0.8, 0.0),
			text,
			position,
		}
	}

	pub fn font_size(mut self, font_size: f32) -> Self{
		self.text.set_size(font_size);
		self
	}

	pub fn text(mut self, text: &str) -> Self{
		self.text.set_content(text);
		self
	}

	pub fn position(mut self, position: Vec2) -> Self{
		self.position = position;
		self
	}

	pub fn color(mut self, color: Color) -> Self{
		self.normal_color = color;
		self
	}

	pub fn hover_color(mut self, color: Color) -> Self{
		self.hover_color = color;
		self
	}

	pub fn pressed_color(mut self, color: Color) -> Self{
		self.pressed_color = color;
		self
	}

	pub fn visible(mut self, visible: bool) -> Self{
		self.visible = visible;
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
			self.color = self.normal_color;
			return true;
		}
		false
	}

	pub fn update(&mut self, ctx: &mut Context){
		if self.visible{
			let mouse_position = glm::round(&input::get_mouse_position(ctx));
			let bounds = self.text.get_bounds(ctx).unwrap();

			if is_inside_hover_area(self.centered, self.position, bounds, mouse_position) {
				if input::is_mouse_button_down(ctx, MouseButton::Left){
					self.color = self.pressed_color;
				}else{
					self.color = self.hover_color;
				}
				if input::is_mouse_button_released(ctx, MouseButton::Left){
					self.pressed = true;
				}
			}else{
				self.color = self.normal_color;
			}
		}
	}

}

impl Drawable for TextButton {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		if self.visible{
			let mut params = params.into();

			params.position = self.position;
			params.color = self.color;
			if self.centered{
				let bounds = self.text.get_bounds(ctx).unwrap();
				params.origin = Vec2::new(bounds.width/2.0,bounds.height/2.0);
			}

			self.text.draw(ctx, params)
		}
	}
}

fn is_inside_hover_area(centered: bool, draw_position: Vec2, area: Rectangle, position: Vec2) -> bool{
	let mut pos_x = draw_position.x;
	let mut pos_y = draw_position.y;
	if centered {
		pos_x -= area.width / 2.0;
		pos_y -= area.height / 2.0;
	}

	!(position.x < area.x + pos_x ||
		position.y < area.y + pos_y ||
		position.x > area.x + pos_x + area.width ||
		position.y > area.y + pos_y + area.height
	)
}

