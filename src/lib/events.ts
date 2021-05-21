import { WebviewWindow } from '@tauri-apps/api/window';

addEventListener('keydown', async event => {
	// Create a new window
	if (event.metaKey && event.key === 'n') {
		// @ts-ignore - marked private
		return new WebviewWindow(
			Math.random().toString(36).slice(2)
		);
	}
});
