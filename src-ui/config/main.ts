import {invoke} from "@tauri-apps/api";

enum DistributionType {
	FromBim,
	Users
}

enum TransitionType {
	FromBim,
	Users
}

type ScenarioCfg = {
	files: string[],
	logger_config: string,
	distribution: {
		distribution_type: DistributionType,
		density: number,
		special: {
			uuid: string[],
			density: number,
			comment: string
		}[]
	},
	transition: {
		transition_type: TransitionType,
		doorway_in: number,
		doorway_out: number,
		special: {
			uuid: string[],
			width: number,
			comment: string
		}[]
	},
	modeling: {
		step: number,
		max_speed: number,
		max_density: number,
		min_density: number
	}
}

(document.getElementById('start-btn') as HTMLElement).addEventListener('click', async _ => {
	try {
		let config = await invoke('read_config') as ScenarioCfg;
		let filesListElement = document.createElement('ul');
		let fileElements = config.files.map(file => {
			const fileElement = document.createElement('li');
			fileElement.innerText = file;
			return fileElement;
		});
		filesListElement.append(...fileElements);
		const outputElement = (document.querySelector('#config-output') as HTMLElement);
		outputElement.innerHTML = '';
		outputElement.appendChild(filesListElement);
	} catch (errorMessage) {
		(document.querySelector('#config-output') as HTMLElement).innerHTML = `
			<p>
				${typeof errorMessage === 'string' ? errorMessage : "Произошла неизвестная ошибка"}
			</p>
		`;
	}
});