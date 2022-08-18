use std::process::Command;
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
	unsafe { run() }
}