use cli::{read_cli_args};
use configuration::load_cfg;

fn main() {
	let cli_params = read_cli_args().expect("Не удалось прочитать аргументы командной строки");
	let scenario = load_cfg(&cli_params.scenario_file).expect("Не удалось прочитать файл конфигурации сценария");
	println!("{:#?}", &scenario);
}