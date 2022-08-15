extern crate serde_json;

use std::error::Error;
use std::fs;
use std::process;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionType {
	FromBim,
	Uniform
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
	FromBim,
	Users
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionSpecial {
	pub uuid: Vec<String>,
	pub density: f64,
	#[serde(rename="_comment")]
	pub comment: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionSpecial {
	pub uuid: Vec<String>,
	pub width: f64,
	#[serde(rename="_comment")]
	pub comment: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
	#[serde(rename="type")]
	pub distribution_type: DistributionType,
	pub density: f64,
	pub special: Vec<DistributionSpecial>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
	#[serde(rename="type")]
	pub transitions_type: TransitionType,
	#[serde(rename="doorwayin")]
	pub doorway_in: f64,
	#[serde(rename="doorwayout")]
	pub doorway_out: f64,
	pub special: Vec<TransitionSpecial>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modeling {
	pub step: f64,
	#[serde(rename="speed_max")]
	pub max_speed: f64,
	#[serde(rename="density_max")]
	pub max_density: f64,
	#[serde(rename="density_min")]
	pub min_density: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioCfg {
	#[serde(rename="bim")]
	pub files: Vec<String>,
	pub distribution: Distribution,
	#[serde(rename="transits")]
	pub transition: Transition,
	pub modeling: Modeling
}

pub fn load_cfg (path_to_file: &str) -> Result<ScenarioCfg, Box<dyn Error>> {
	let json_content = fs::read_to_string(path_to_file).unwrap_or_else(|err| {
		eprintln!("Ошибка чтения файла конфигурации сценария {}: {}", path_to_file, err);
		process::exit(1);
	});

	let cfg: ScenarioCfg = serde_json::from_str(&json_content).unwrap_or_else(|err| {
		eprintln!("Ошибка десериализации файла конфигурации сценария {}: {}", path_to_file, err);
		process::exit(1);
	});

	Ok(cfg)
}