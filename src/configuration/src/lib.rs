extern crate serde_json;

use std::error::Error;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum DistributionType {
	FromBim,
	Uniform
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum TransitionType {
	FromBim,
	Users
}

#[derive(Serialize, Deserialize)]
pub struct DistributionSpecial {
	pub uuid: Vec<String>,
	pub density: f64,
	#[serde(rename(deserialize = "_comment"))]
	pub comment: String
}

#[derive(Serialize, Deserialize)]
pub struct TransitionSpecial {
	pub uuid: Vec<String>,
	pub width: f64,
	#[serde(rename(deserialize = "_comment"))]
	pub comment: String
}

#[derive(Serialize, Deserialize)]
pub struct Distribution {
	#[serde(rename(deserialize = "type"))]
	pub distribution_type: DistributionType,
	pub density: f64,
	pub special: Vec<DistributionSpecial>
}

#[derive(Serialize, Deserialize)]
pub struct Transition {
	#[serde(rename(deserialize = "type"))]
	pub transitions_type: TransitionType,
	#[serde(rename(deserialize = "doorwayin"))]
	pub doorway_in: f64,
	#[serde(rename(deserialize = "doorwayout"))]
	pub doorway_out: f64,
	pub special: Vec<TransitionSpecial>
}

#[derive(Serialize, Deserialize)]
pub struct Modeling {
	pub step: f64,
	#[serde(rename(deserialize = "speed_max"))]
	pub max_speed: f64,
	#[serde(rename(deserialize = "density_max"))]
	pub max_density: f64,
	#[serde(rename(deserialize = "density_min"))]
	pub min_density: f64
}

#[derive(Serialize, Deserialize)]
pub struct ScenarioCfg {
	#[serde(rename(deserialize = "bim"))]
	pub files: Vec<String>,
	#[serde(rename(deserialize = "logger_configure"))]
	pub logger_config: String,
	pub distribution: Distribution,
	#[serde(rename(deserialize = "transits"))]
	pub transition: Transition,
	pub modeling: Modeling
}

#[derive(Debug, Serialize)]
pub struct LoadCfgError {
	pub message: String
}

impl std::fmt::Display for LoadCfgError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "LoadCfgError: {}", self.message)
	}
}

impl Error for LoadCfgError {}

pub fn load_cfg (path_to_file: &str) -> Result<ScenarioCfg, LoadCfgError> {
	match Path::new(path_to_file).exists() {
		true => {
			let json_content = fs::read_to_string(path_to_file).unwrap_or_else(|err| {
				panic!("Ошибка чтения файла конфигурации сценария {}: {}", path_to_file, err)
			});

			let cfg: ScenarioCfg = serde_json::from_str(&json_content).unwrap_or_else(|err| {
				panic!("Ошибка десериализации файла конфигурации сценария {}: {}", path_to_file, err)
			});

			Ok(cfg)
		},
		false => Err(LoadCfgError {
			message: format!("Не удалось найти указанный файл: {}", path_to_file)
		})
	}
}