import { invoke } from "@tauri-apps/api";

document.getElementById('file_input').addEventListener('change', e => {
	console.log(e.target.value);
});

document.getElementById('start-btn').addEventListener('click', async _ => {
	document.querySelector('#config-text').innerHTML = await invoke('read_config');
});