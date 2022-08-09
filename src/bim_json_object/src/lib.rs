#![allow(non_camel_case_types)]

use std::fs;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use libc::{c_char, c_ulonglong, c_double};
use json_object::parse_building_from_json;

#[repr(C)]
pub enum bim_element_sign_t_rust
{
	ROOM,         //< Указывает, что элемент здания является помещением/комнатой
	STAIRCASE,    //< Указывает, что элемент здания является лестницей
	DOOR_WAY,     //< Указывает, что элемент здания является проемом (без дверного полотна)
	DOOR_WAY_INT, //< Указывает, что элемент здания является дверью, которая соединяет
					   // < два элемента: ROOM и ROOM или ROOM и STAIR
	DOOR_WAY_OUT, //< Указывает, что элемент здания является эвакуационным выходом
	OUTSIDE,      //< Указывает, что элемент является зоной вне здания
	UNDEFINDED    //< Указывает, что тип элемента не определен
}

// Количество символов в UUID + NUL символ
#[repr(C)]
struct uuid_t_rust {
	x: [c_char; 36 + 1] // массив из char заменён на обычную строку
}

#[repr(C)]
pub struct point_t_rust
{
	x: c_double,
	y: c_double
}

#[repr(C)]
pub struct polygon_t_rust
{
	numofpoints: c_ulonglong,
	points: *mut point_t_rust
}

// Структура, описывающая элемент
#[repr(C)]
pub struct bim_json_element_t_rust {
	uuid: uuid_t_rust,            //< [JSON] UUID идентификатор элемента
	name: *const c_char,          //< [JSON] Название элемента
	polygon: *mut polygon_t_rust,      //< [JSON] Полигон элемента
	outputs: *mut uuid_t_rust,    //< [JSON] Массив UUID элементов, которые являются соседними к элементу
	id: c_ulonglong,              //< Внутренний номер элемента (генерируется)
	numofpeople: c_ulonglong,     //< [JSON] Количество людей в элементе
	numofoutputs: c_ulonglong,    //< Количество связанных с текущим элементов
	size_z: c_double,             //< [JSON] Высота элемента
	z_level: c_double,            //< Уровень, на котором находится элемент
	sign: bim_element_sign_t_rust //< [JSON] Тип элемента
}

// Структура поля, описывающего географическое положение объекта
#[repr(C)]
pub struct bim_json_address_t_rust {
	city: *const c_char,           //< [JSON] Название города
	street_address: *const c_char, //< [JSON] Название улицы
	add_info: *const c_char        //< [JSON] Дополнительная информация о местоположении объекта
}

// Структура, описывающая этаж
#[repr(C)]
pub struct bim_json_level_t_rust {
	name: *const c_char,                      //< [JSON] Название этажа
	elements: *mut bim_json_element_t_rust, //< [JSON] Массив элементов, которые принадлежат этажу
	z_level: c_double,                        //< [JSON] Высота этажа над нулевой отметкой
	numofelements: c_ulonglong                //< Количство элементов на этаже
}

// Структура, описывающая здание
#[repr(C)]
pub struct bim_json_object_t_rust {
	address: *mut bim_json_address_t_rust, //< [JSON] Информация о местоположении объекта
	name: *const c_char,                   //< [JSON] Название здания
	levels: *mut bim_json_level_t_rust,    //< [JSON] Массив уровней здания
	numoflevels: c_ulonglong               //< Количество уровней в здании
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bim_json_new_rust(path_to_file: *const c_char) -> *const bim_json_object_t_rust {
	let building = unsafe { parse_building_from_json(CStr::from_ptr(path_to_file).to_str().unwrap()).expect("Ошибка при парсинге здания") };
	let mut bim_element_rs_id: u64 = 0;
	let mut bim_element_d_id: u64 = 0;

	let bim_json_object = bim_json_object_t_rust {
		address: Box::into_raw(Box::new(bim_json_address_t_rust {
			city: CString::new(building.address.city).unwrap().into_raw(),
			street_address: CString::new(building.address.street_address).unwrap().into_raw(),
			add_info: CString::new(building.address.add_info).unwrap().into_raw()
		})),
		numoflevels: c_ulonglong::try_from(building.levels.len()).unwrap(),
		name: CString::new(building.name_building).unwrap().into_raw(),
		levels: {
			let mut levels = building.levels.iter().map(|level| {
				bim_json_level_t_rust {
					name: CString::new(level.name.clone()).unwrap().into_raw(),
					numofelements: c_ulonglong::try_from(level.build_elements.len()).unwrap(),
					z_level: level.z_level,
					elements: {
						let mut build_elements = level.build_elements.iter().enumerate().map(|(i, element)| {
							bim_json_element_t_rust {
								uuid: uuid_t_rust {
									x: {
										let mut char_vec = element.id.clone().chars().map(|c| match c.is_ascii() {
											true => c as c_char,
											false => panic!("uuid символ вне диапазона ASCII")
										}).collect::<Vec<c_char>>();
										char_vec.push(0 as c_char);
										char_vec.try_into().unwrap_or_else(|v| panic!("Не удалось преобразовать uuid в массив char длиной 37: {:?}", v))
									}
								},
								name: CString::new(element.name.clone()).unwrap().into_raw(),
								id: match element.sign.as_str() { // TODO: сделать отсчёт id с нуля
									"Room" => {
										bim_element_rs_id += 1;
										bim_element_rs_id
									},
									"Staircase" => {
										bim_element_rs_id += 1;
										bim_element_rs_id
									},
									"DoorWay" => {
										bim_element_d_id += 1;
										bim_element_d_id
									},
									"DoorWayInt" => {
										bim_element_d_id += 1;
										bim_element_d_id
									},
									"DoorWayOut" => {
										bim_element_d_id += 1;
										bim_element_d_id
									}
									_ => 0
								},
								size_z: element.size_z,
								z_level: level.z_level,
								numofpeople: element.number_of_people,
								numofoutputs: c_ulonglong::try_from(element.outputs.len()).unwrap(),
								sign: match element.sign.as_str() {
									"Room" => bim_element_sign_t_rust::ROOM,
									"Staircase" => bim_element_sign_t_rust::STAIRCASE,
									"DoorWay" => bim_element_sign_t_rust::DOOR_WAY,
									"DoorWayInt" => bim_element_sign_t_rust::DOOR_WAY_INT,
									"DoorWayOut" => bim_element_sign_t_rust::DOOR_WAY_OUT,
									_ => bim_element_sign_t_rust::UNDEFINDED
								},
								outputs: {
									let mut outputs = element.outputs.iter().map(|output| {
										uuid_t_rust {
											x: {
												let mut char_vec = output.clone().chars().map(|c| match c.is_ascii() {
													true => c as c_char,
													false => panic!("uuid символ вне диапазона ASCII: {}", c)
												}).collect::<Vec<c_char>>();
												char_vec.push(0 as c_char);
												char_vec.try_into().unwrap_or_else(|v| panic!("Не удалось преобразовать uuid в массив char длиной 37: {:?}", v))
											}
										}
									}).collect::<Vec<uuid_t_rust>>();

									let ptr = outputs.as_mut_ptr();
									std::mem::forget(outputs);

									ptr
								},
								polygon: Box::into_raw(Box::new(polygon_t_rust {
									numofpoints: c_ulonglong::try_from(element.xy[0].points.len()).unwrap(),
									points: {
										let mut points = element.xy[0].points.iter().map(|point| {
											point_t_rust {
												x: point.x,
												y: point.y
											}
										}).collect::<Vec<point_t_rust>>();

										let ptr = points.as_mut_ptr();
										std::mem::forget(points);

										ptr
									}
								}))
							}
						}).collect::<Vec<bim_json_element_t_rust>>();

						let build_elements_ptr = build_elements.as_mut_ptr();
						std::mem::forget(build_elements);

						build_elements_ptr
					}
				}
			}).collect::<Vec<bim_json_level_t_rust>>();

			let levels_ptr = levels.as_mut_ptr();
			std::mem::forget(levels);

			levels_ptr
		}
	};


	Box::into_raw(Box::new(bim_json_object))
}