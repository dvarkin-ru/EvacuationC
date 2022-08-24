import * as ReactDOM from 'react-dom/client';
import { invoke } from "@tauri-apps/api";
import App from "./App";

(document.getElementById('file_input') as HTMLInputElement).addEventListener('change', e => {
	console.log((e.target as HTMLInputElement).value);
});

(document.getElementById('start-btn') as HTMLElement).addEventListener('click', async _ => {
	(document.querySelector('#config-text') as HTMLElement).innerHTML = await invoke('read_config');
});

const rootElement = document.querySelector('#root');
if (rootElement) {
    const root = ReactDOM.createRoot(rootElement);
    root.render(<App />);
} else {
    console.error('Root element not found');
}
