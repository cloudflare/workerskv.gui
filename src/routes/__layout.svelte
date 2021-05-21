<script>
	import '../index.css';
	import { onMount } from 'svelte';
	onMount(() => import('$lib/events'));
</script>

<slot/>
