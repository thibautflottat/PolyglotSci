import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter()
	},

	vitePlugin: {

			inspector: {
				holdMode: true,
				showToggleButton: 'always',
				toggleButtonPos: 'bottom-right'
			}
		
	}
};

export default config;
