import {invoke} from "@tauri-apps/api";

(document.getElementById('start-btn') as HTMLElement).addEventListener('click', async _ => {
    (document.querySelector('#config-text') as HTMLElement).innerHTML = await invoke('read_config');
});