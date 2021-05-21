<script>
	import * as utils from '$lib/utils';

	export let value = '';
</script>

<time>
	{ value ? utils.date(value) : 'Never' }
</time>

<style>
	time {
		font-weight: 400;
	}
</style>
