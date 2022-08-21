use std::fs;
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
	fs::create_dir_all("result").expect("Не удалось создать папку result");
	unsafe { run() }
}