extern crate serde_json;

use std::error::Error;
use std::fs;
use std::process;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DistributionType {
	FromBim,
	Uniform
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TransitionType {
	FromBim,
	Users
}

#[derive(Debug, Serialize, Deserialize)]
struct DistributionSpecial {
	uuid: Vec<String>,
	density: f64,
	#[serde(rename="_comment")]
	comment: String
}

#[derive(Debug, Serialize, Deserialize)]
struct TransitionSpecial {
	uuid: Vec<String>,
	width: f64,
	#[serde(rename="_comment")]
	comment: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Distribution {
	#[serde(rename="type")]
	distribution_type: DistributionType,
	density: f64,
	special: Vec<DistributionSpecial>
}

#[derive(Debug, Serialize, Deserialize)]
struct Transition {
	#[serde(rename="type")]
	transitions_type: TransitionType,
	#[serde(rename="doorwayin")]
	doorway_in: f64,
	#[serde(rename="doorwayout")]
	doorway_out: f64,
	special: Vec<TransitionSpecial>
}

#[derive(Debug, Serialize, Deserialize)]
struct Modeling {
	step: f64,
	#[serde(rename="speed_max")]
	max_speed: f64,
	#[serde(rename="density_max")]
	max_density: f64,
	#[serde(rename="density_min")]
	min_density: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioCfg {
	#[serde(rename="bim")]
	files: Vec<String>,
	distribution: Distribution,
	#[serde(rename="transits")]
	transition: Transition,
	modeling: Modeling
}

pub fn load_cfg (filename: &String) -> Result<ScenarioCfg, Box<dyn Error>> {
	let json_content = fs::read_to_string(filename)
		.expect("Не удалось прочитать файл конфигурации сценария");
	let cfg: ScenarioCfg = serde_json::from_str(&json_content).unwrap_or_else(|err| {
		println!("Ошибка чтения файла конфигурации сценария: {}", err);
		process::exit(1);
	});

	Ok(cfg)
}