<script>
	export let value;
	$: empty = value === void 0;
</script>

<pre class:empty><code>{value || '<empty>'}</code></pre>

<style>
	.empty {
		-webkit-user-select: none;
		pointer-events: none;
		font-style: italic;
		opacity: 0.4;
	}
</style>
