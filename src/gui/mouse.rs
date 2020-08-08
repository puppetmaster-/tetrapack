use tetra::graphics::{Texture, Drawable, DrawParams};
use tetra::{Context, input};

pub struct Mouse{
	texture: Texture,
	visible: bool,
}

impl Mouse{
	pub fn new(ctx: &mut Context) -> tetra::Result<Mouse>{
		let texture = Texture::from_file_data(ctx, include_bytes!("../../resources/cursor.png"))?;
		Ok(Mouse{
			texture,
			visible: true,
		})
	}

	pub fn set_visible(&mut self, visible: bool){
		self.visible = visible;
	}

	pub fn set_texture(&mut self, texture: Texture){
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
			params.position = input::get_mouse_position(ctx).round();
			self.texture.draw(ctx, params)
		}
	}
}