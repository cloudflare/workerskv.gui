<script>
	export let label = '';
	export let color = '';

	let className = '';
	export { className as class };
</script>

<span class={className} style="--c: {color}">{label}</span>

<style>
	span {
		font-weight: bold;
	}

	span::before {
		content: '';
		position: relative;
		display: inline-block;
		margin-right: 0.25rem;
		background: var(--c, transparent);
		border-radius: 50%;
		height: 0.5rem;
		width: 0.5rem;
	}
</style>
