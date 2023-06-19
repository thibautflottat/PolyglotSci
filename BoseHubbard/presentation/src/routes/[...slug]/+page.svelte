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
	<Slide>
		<h2 class="pt-[10%]">Rust</h2>
		<hr class="border-2 border-blue-300" />
		<p class="text-left text-3xl pt-20">Mateus Rodolfo & Thibaut Flottat<br />
		<em>23 juin 2023</em>
		</p>
		<img class="scale-75 !ml-[75%]" src="/img/log_simatlab_coul-removebg-preview.png" alt="logo" />
	</Slide>

	<Slide id="fragments">
		<h3>Goals and motivation</h3>
		<p class="text-left">Anion Exchange Membranes (AEM) are semipermeable membranes made of hydrophobic backbone and hydrophilic side chains (spacers) and designed to conduct anions (in our case $OH^-$). AEM are used as solid electrolyte in hydrogen fuel cells and water electrolysers.</p>

		<div class="r-stack">
		<img src="/aem.webp" class="fragment w-2/3 center fade-out current-visible" alt="aem" data-fragment-index="0" />
		<div class="grid grid-cols-2 gap-4 fragment" data-fragment-index="1">
			<h5>Pros</h5>
			<h5>Cons</h5>
			<ul class="fragment">
				<li class="text-lime-300 text-4xl">High-cost noble metal catalyst ($Pt$) is not required</li>
				<li class="text-lime-300 text-4xl">AEM based on aromatic polymer backbones</li>
			</ul>
			
			<ul>
				<li class="fragment text-red-300 text-4xl">Lower efficiency</li>
				<li class="fragment text-red-300 text-4xl">Chemical deterioration (Hofmann elimination, nucleophilic substitution, E1 elimination)</li>
			</ul>
		</div>
		</div>
	</Slide>

	<Slide>
		
		<Slide transition='fade'>
			<h3>Ultra-cold atoms</h3>
		</Slide>
		<Slide transition='fade'>
			<h3>Ultra-cold atoms on optical lattices</h3>
		</Slide>
		<Slide transition='fade'>
			<h3>Bose-Hubbard Hamiltonian</h3>
		</Slide>
	</Slide>


	<Slide>
		<h3 class=" pt-[25%]">Thank you for your attention!</h3>
	</Slide>

	</RevealJsContext>
	{/if}
</div>
