#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fs;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
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
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let close = CustomMenuItem::new("close".to_string(), "Close");
	let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
	let menu = Menu::new()
		.add_native_item(MenuItem::Copy)
		.add_item(CustomMenuItem::new("hide", "Hide"))
		.add_submenu(submenu);

	// runtime windows initialize
	tauri::Builder::default()
		.setup(|app| {
			/*let main_window = tauri::WindowBuilder::new(
				app,
				"evacuation",
				tauri::WindowUrl::App("index.html".into())
			).menu(menu).build().unwrap();

			let config_window = tauri::WindowBuilder::new(
				app,
				"configuration",
				tauri::WindowUrl::App("config.html".into())
			).build().unwrap();*/
			Ok(())
		})
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