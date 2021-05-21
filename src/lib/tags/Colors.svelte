<script>
	let colors = [
		'#fa5252', // red
		'#e64980', // pink
		'#be4bdb', // grape
		'#7950f2', // violet
		'#4c6ef5', // indigo
		'#228be6', // blue
		'#15aabf', // cyan
		'#12b886', // teal
		'#40c057', // green
		'#82c91e', // lime
		'#fab005', // yellow
		'#fd7e14', // orange
	];

	export let id = '';
	export let name = '';
	export let value = colors[0];
</script>

<input {id} {name} type="hidden" bind:value >

<div style="--n: {colors.length}">
	{#each colors as color (color)}
	<span
		class:active={color === value}
		on:click={() => value=color}
		style="--c: {color}"
	></span>
	{/each}
</div>

<style>
	div {
		display: grid;
		grid-template-columns: repeat(var(--n), 1fr);
		column-gap: 4px;
	}

	span {
		background: var(--c);
		border-radius: 50%;
		height: 1.125rem;
		width: 1.125rem;
	}

	span.active {
		box-shadow: 0 0 0 1px #e9ecef, 0 0 0 3px var(--c);
	}
</style>
