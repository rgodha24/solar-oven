<script lang="ts">
	import { getGraphData } from '$lib';

	const data = getGraphData({
		oven: ['BCS', 'DoubleMylar', 'C', 'C', 'FG30', 'Trapezoidal', 'TF', 4],
		reflector_ml: 3,
		data_type: 'Score'
	});
</script>

{#await data}
	loading...
{:then d}
	{#each d as item}
		<p>{JSON.stringify(item)}</p>
	{/each}
{:catch e}
	{JSON.stringify(e)}
{/await}
