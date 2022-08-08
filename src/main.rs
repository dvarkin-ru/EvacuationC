use cli::read_cli_args;
use configuration::load_cfg;
use json_object::parse_building_from_json;

fn main() {
	let cli_params = read_cli_args()
		.expect("Не удалось прочитать аргументы командной строки");
	let scenario = load_cfg(&cli_params.scenario_file)
		.expect("Не удалось прочитать файл конфигурации сценария");

	for building_filename in scenario.files {
		let building = parse_building_from_json(&building_filename)
			.expect("Не удалось прочитать файл здания");

		println!("{:#?}", &building_filename);
	}
}