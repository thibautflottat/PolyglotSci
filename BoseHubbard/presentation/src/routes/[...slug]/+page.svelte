<script lang="ts">
	import { onMount } from 'svelte';
	import { Code, RevealJsContext, Slide } from '$lib';
	import PlotlyGraph from '$lib/PlotlyGraph.svelte';
	import type Reveal from 'reveal.js';

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
	<Slide id="fragments">
		<h2 class="pt-[10%]"><span class="fragment highlight-current-blue" data-fragment-index="3">Investigating the influence of spacer length</span> on <span class="fragment highlight-current-blue" data-fragment-index="1">Anion Exchange Membranes</span> properties using <span class="fragment highlight-current-blue" data-fragment-index="2">reactive molecular dynamics</span></h2>
		<hr class="border-2 border-blue-300" />
		<p class="text-left text-3xl pt-20">Mateus Rodolfo<br />Thibaut Flottat<br />
		<em>23 juin 2023</em>
		</p>
		<img class="scale-75 !ml-[75%]" src="/img/log_simatlab_coul-removebg-preview.png" alt="logo" />
	</Slide>

	<Slide>
		<h3 class=" pt-[25%]">Thank you for your attention!</h3>
	</Slide>


	</RevealJsContext>
	{/if}
</div>
