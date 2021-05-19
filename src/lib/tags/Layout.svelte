<script>
	// export let resize = false;
	let className = '';
	export { className as class }

	export let wider = false;
</script>

<main class={className}>
	<aside class:wider>
		<slot name="aside"/>
	</aside>

	<section>
		<slot name="content"/>
	</section>
</main>

<style>
	main {
		display: flex;
		height: 100vh;
		width: 100vw;
	}

	aside {
		width: clamp(220px, 30%, 360px);
		border-right: var(--border);
		background-color: #f8f9fa;
	}

	aside.wider {
		width: clamp(300px, 40%, 480px);
	}

	aside > :global(*),
	section > :global(*) {
		width: 100%;
	}

	section {
		flex: 1;
		background-color: #f1f3f5;
	}
</style>
