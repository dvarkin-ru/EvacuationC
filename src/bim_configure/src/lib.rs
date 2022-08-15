#![allow(non_camel_case_types)]

use std::ffi::{CStr};
use libc::{c_char, c_uchar, c_float};
use configuration::load_cfg;

// Количество символов в UUID + NUL символ
#[repr(C)]
pub struct uuid_t_rust {
	pub x: [c_char; 36 + 1] // массив из char заменён на обычную строку
}

#[repr(C)]
pub enum distribution_type_rust {
	distribution_from_bim,
	distribution_uniform
}

#[repr(C)]
pub enum transits_width_type_rust {
	transits_width_from_bim,
	transits_width_users
}

#[repr(C)]
pub struct special_t_rust
{
	pub uuid: uuid_t_rust,
	pub num_of_uuids: c_uchar,
	pub value: c_float
}

#[repr(C)]
pub struct bim_cfg_distribution_t_rust
{
	pub r#type: distribution_type_rust,
	pub density: c_float,
	pub special: *mut special_t_rust,
	pub num_of_special_blocks: c_uchar
}

#[repr(C)]
pub struct bim_cfg_transitions_width_t_rust
{
	pub r#type: transits_width_type_rust,
	pub doorwayin: c_float,
	pub doorwayout: c_float,
	pub special: *mut special_t_rust,
	pub num_of_special_blocks: c_uchar
}
#[repr(C)]
pub struct bim_cfg_modeling_t_rust
{
	pub step: c_float,
	pub speed_max: c_float,
	pub density_min: c_float,
	pub density_max: c_float
}

#[repr(C)]
pub struct bim_cfg_file_name_t_rust
{
	pub x: [c_char; 256]
}

#[repr(C)]
pub struct bim_cfg_scenario_t_rust
{
	pub bim_jsons: *mut bim_cfg_file_name_t_rust,
	pub logger_configure: bim_cfg_file_name_t_rust,
	pub num_of_bim_jsons: c_uchar,
	// pub distribution: bim_cfg_distribution_t,
	// pub transits: bim_cfg_transitions_width_t,
	// pub modeling: bim_cfg_modeling_t
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bim_cfg_load_rust(path_to_file: *const c_char) -> *const bim_cfg_scenario_t_rust {
	let config = unsafe { load_cfg(CStr::from_ptr(path_to_file).to_str().unwrap())
		.expect("Ошибка парсинга конфигурации сценария") };
	let scenario_config = bim_cfg_scenario_t_rust {
		bim_jsons: {
			let mut files = config.files.iter().map(|filename| {
				bim_cfg_file_name_t_rust {
					x: {
						let mut char_arr: [c_char; 256] = [0; 256];
						for (i, c) in filename.chars().enumerate() {
							match c.is_ascii() {
								true => char_arr[i] = c as c_char,
								false => panic!("uuid символ вне диапазона ASCII")
							}
						}

						char_arr
					}
				}
			}).collect::<Vec<bim_cfg_file_name_t_rust>>();

			let ptr = files.as_mut_ptr();
			std::mem::forget(files);
			ptr
		},
		num_of_bim_jsons: c_uchar::try_from(config.files.len()).expect("Количество файлов в конфигурации больше 255"),
		logger_configure: bim_cfg_file_name_t_rust {
			x: {
				let mut char_arr: [c_char; 256] = [0; 256];
				for (i, c) in config.loggerConfig.chars().enumerate() {
					match c.is_ascii() {
						true => char_arr[i] = c as c_char,
						false => panic!("uuid символ вне диапазона ASCII")
					}
				}

				char_arr
			}
		}
	};

	Box::into_raw(Box::new(scenario_config))
}