use tetra::{
	Event,
	input::{GamepadButton, Key}
};

#[allow(dead_code)]
pub fn is_confirmation(event: &Event) -> bool{
	match event{
		Event::KeyReleased{key: Key::Space} | Event::KeyReleased {key: Key::Enter} => true,
		Event::GamepadButtonReleased{ id: _, button } => button == &GamepadButton::A || button == &GamepadButton::Start,
		_ => false
	}
}

#[allow(dead_code)]
pub fn is_cancel(event: &Event) -> bool{
	match event {
		Event::KeyReleased { key: Key::Escape } | Event::KeyReleased { key: Key::Backspace } => true,
		Event::GamepadButtonReleased {id: _, button: GamepadButton::Back } => true,
		_ => false,
	}
}

#[allow(dead_code)]
pub fn is_any_key(event: &Event) -> bool{
	match event{
		Event::KeyReleased {..} | Event::GamepadButtonReleased{..} => true,
		_ => false
	}
}

#[allow(dead_code)]
pub fn is_mouse_event(event: &Event) -> bool{
	match event{
		Event::MouseMoved {..} | Event::MouseButtonPressed {..} | Event::MouseButtonReleased{..} => true,
		_ => false
	}
}