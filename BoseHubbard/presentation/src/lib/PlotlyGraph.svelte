<!-- <script context="module">
	const graphs = import.meta.glob(
		'/graphs/*.json',
		{
			import: 'default',
			eager: true
		}
	);
</script> -->

<script lang="ts">

	export let src:string;
  export let id:string;
  // let graph:any;
  // $: graph = graphs[src];
  import { onMount } from 'svelte';

  let plotData;

  onMount(async () => {
        const Plotly = await import('plotly.js-dist-min')
        // const url = `/src/lib/plotly/${src}.json`
        const response = await fetch(src);
        plotData = await response.json();
      Plotly.newPlot(id, plotData.data, plotData.layout);    
      

      // Update the width of the Plotly chart
      var width = 1600; // Set the desired width value in pixels

      // Update the layout of the Plotly chart
      Plotly.relayout(id, { width: width });
    });

</script>

<div id={id}></div>

