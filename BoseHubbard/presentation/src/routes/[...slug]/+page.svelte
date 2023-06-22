<script lang="ts">
	import { onMount } from 'svelte';
	import { Code, RevealJsContext, Slide } from '$lib';
	import type Reveal from 'reveal.js';

	import Slide1 from '$lib/Slides/slide1.svelte';
	import Slide2 from '$lib/Slides/slide2.svelte';
	import Slide3 from '$lib/Slides/slide3.svelte';
	import Slide4 from '$lib/Slides/slide4.svelte';
	import Slide6 from '$lib/Slides/slide6.svelte';

	// The highlight plugin requires a stylesheet
	import 'reveal.js/dist/theme/black.css';
	import 'reveal.js/plugin/highlight/monokai.css';

	let plugins: Reveal.PluginFunction[];

	onMount(async () => {
		plugins = [
			await import('reveal.js/plugin/highlight/highlight').then(res => res.default),
			// await import('reveal.js/plugin/markdown/markdown').then(res => res.default),
			await import('reveal.js/plugin/math/math').then(res => res.default)
		]

	})
</script>

<!-- a container with fixed size is required -->
<div style:width="100%" style:height="100vh">
	{#if plugins}
	<RevealJsContext
		options={{
			width: 1600,
			height: 900,	
			margin: 0.1,
			// transition: 'fade',
			controls: true,
			progress: true,
			center: false,
			hash: true,
			plugins
		}}
	>

	<Slide1 />
	<Slide2 />
	<Slide3 />
	<Slide4 />
	<Slide6 />

	</RevealJsContext>
	{/if}
</div>
