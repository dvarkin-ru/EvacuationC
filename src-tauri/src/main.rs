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
		.invoke_handler(tauri::generate_handler![read_config])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn read_config() -> configuration::ScenarioCfg {
	let config = configuration::load_cfg("../scenario.json")
		.expect("Не удалось прочитать файл конфигурации");
	config
}