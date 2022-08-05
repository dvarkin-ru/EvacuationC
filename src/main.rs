extern "C" {
	fn run() -> i32;
}

fn main() {
	unsafe { run(); };
}