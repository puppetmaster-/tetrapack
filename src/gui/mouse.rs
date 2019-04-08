use tetra::graphics::{Texture, Drawable, DrawParams};
use tetra::{Context, input, glm};

pub struct Mouse{
	texture: Texture,
	visible: bool,
}

impl Mouse{
	pub fn new(texture: Texture) -> Self{
		Mouse{
			texture,
			visible: true,
		}
	}

	pub fn visible(&mut self, visible: bool){
		self.visible = visible;
	}

	pub fn texture(&mut self, texture: Texture){
		self.texture = texture;
	}
}

impl Drawable for Mouse {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		if self.visible{
			let mut params = params.into();
			params.position = glm::round(&input::get_mouse_position(ctx));
			self.texture.draw(ctx, params)
		}
	}
}