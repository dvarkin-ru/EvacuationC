import * as ReactDOM from 'react-dom/client';
import App from './App';

(document.getElementById('file_input') as HTMLInputElement).addEventListener('change', e => {
	console.log((e.target as HTMLInputElement).value);
});

const rootElement = document.querySelector('#root');
if (rootElement) {
	const root = ReactDOM.createRoot(rootElement);
	root.render(<App />);
} else {
	console.error('Root element not found');
}
