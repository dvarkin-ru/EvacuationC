import { invoke } from "@tauri-apps/api";

invoke('greet', { name: 'World' })
	.then(response => console.log(response));

const fileInput = document.getElementById('file_input');

fileInput.addEventListener('change', e => {
	console.log(e.target.value);
});