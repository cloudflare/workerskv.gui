import { exit } from '@tauri-apps/api/process';

addEventListener('keydown', async event => {
	if (!event.metaKey) return;

	let key = event.key;
	if (key === 'Meta') return;

	// Exit the application
	if (key === 'q') return await exit(0);

	// Copy text to cipboard
	if (key === 'c') {
		let text = window.getSelection().toString();
		return text.length && document.execCommand('copy');
	}

	// Input events; eg clipboard stuff
	let target = event.target as HTMLElement;
	if (/^(input|textarea)$/i.test(target.nodeName)) {
		// Highlight all text within an input element
		if (key === 'a') return (target as HTMLInputElement).select();

		// Paste content into the text input
		// @see https://github.com/tauri-apps/tauri/issues/1055
		if (key === 'v') return; // document.execCommand('paste');
	}
});
