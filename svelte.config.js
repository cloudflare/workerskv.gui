import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			fallback: 'index.html'
		}),
		files: {
			template: 'src/index.html'
		},
		vite: {
			ssr: {
				noExternal: [
					'@tauri-apps/api',
					'@tauri-apps/api/process',
					'@tauri-apps/api/window',
				]
			},
			optimizeDeps: {
				include: [
					'@tauri-apps/api',
					'@tauri-apps/api/process',
					'@tauri-apps/api/window',
				]
			}
		}
	},
};

export default config;
