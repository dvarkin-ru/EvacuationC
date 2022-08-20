use iui::{
	UI,
	controls
};
use cli;
use configuration;
use json_object;
use bim_cli;
use bim_configure;
use bim_json_object;

extern "C" {
	fn run();
}

fn main() {
	// unsafe { run() }
	let ui = UI::init().expect("Couldn't initialize UI");
	let mut window = controls::Window::new(
		&ui,
		"evacuation",
		800,
		600,
		controls::WindowType::HasMenubar
	);

	window.show(&ui);

	ui.main();
}