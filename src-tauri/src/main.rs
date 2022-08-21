#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

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
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![start])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn start() {
	fs::create_dir_all("result").expect("Не удалось создать папку result");
	unsafe { run() }
	println!("Hello from Rust!");
}