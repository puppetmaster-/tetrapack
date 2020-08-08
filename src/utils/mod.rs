use tetra::ContextBuilder;

pub mod ron;
pub mod timer;
pub mod vecgrid;

#[allow(dead_code)]
pub fn load_context(path: &str) -> ContextBuilder{
	match ron::from_str(path){
		Ok(cb) => cb,
		Err(error) => {
			println!("Failed to load contextBuilder: {}", error);
			std::process::exit(1);
		}
	}
}

#[allow(dead_code)]
fn log_time<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
	let time = std::time::Instant::now();
	let result = f();
	println!("{} {:?}", name, time.elapsed());
	result
}

/*
fn get_size(a: HeavyThing) -> usize {
	let size = a.size();
	std::thread::spawn(move || drop(a));
	size
}
 */

