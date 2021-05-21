import { WebviewWindow } from '@tauri-apps/api/window';

if (typeof window !== 'undefined') {
	addEventListener('keydown', async event => {
		// Create a new window
		if (event.metaKey && event.key === 'n') {
			event.preventDefault();
			// @ts-ignore - marked private
			return new WebviewWindow(
				Math.random().toString(36).slice(2)
			);
		}
	});
}

// webview.once('tauri://error', function (e) {
// 	console.log('an error happened creating the webview window');
// });
