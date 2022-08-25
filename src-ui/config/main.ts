import { invoke } from '@tauri-apps/api';

enum DistributionType {
	FromBim,
	Users
}

enum TransitionType {
	FromBim,
	Users
}

type ScenarioCfg = {
	files: string[];
	logger_config: string;
	distribution: {
		distribution_type: DistributionType;
		density: number;
		special: {
			uuid: string[];
			density: number;
			comment: string;
		}[];
	};
	transition: {
		transition_type: TransitionType;
		doorway_in: number;
		doorway_out: number;
		special: {
			uuid: string[];
			width: number;
			comment: string;
		}[];
	};
	modeling: {
		step: number;
		max_speed: number;
		max_density: number;
		min_density: number;
	};
};

(document.getElementById('start-btn') as HTMLElement).addEventListener('click', async _ => {
	try {
		let config = (await invoke('read_config')) as ScenarioCfg;
		(document.querySelector('.config__files') as HTMLElement).innerHTML = config.files.reduce(
			(filenameElements, pathToFile) => filenameElements.concat(`<li>${pathToFile}</li>`),
			''
		);
	} catch (errorMessage) {
		const configElement = document.querySelector('.config') as HTMLElement;
		configElement.innerHTML = `
			<p>
				${typeof errorMessage === 'string' ? errorMessage : 'Произошла неизвестная ошибка'}
			</p>
		`;
	} finally {
		(document.querySelector('.config') as HTMLDivElement).style.display = 'block';
	}
});
