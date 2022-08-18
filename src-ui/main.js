import { invoke } from "@tauri-apps/api";

invoke('greet', { name: 'World' })
	.then(response => console.log(response));