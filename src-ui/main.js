import { invoke } from "@tauri-apps/api";

invoke('greet', { name: 'World' })
	.then(response => console.log(response));

document.getElementById('file_input').addEventListener('change', e => {
	console.log(e.target.value);
});

document.getElementById('start-btn').addEventListener('click', _ => {
	invoke('start').then(_ => alert('started'));
});