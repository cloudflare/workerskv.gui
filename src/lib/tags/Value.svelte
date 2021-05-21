<script lang="ts">
	import { toObjectURL } from '$lib/utils/mime';
	import type { Category } from '$lib/utils/mime';

	export let value: string;
	export let mimetype: Nullable<Category>;

	let empty: boolean, src: string;

	$: {
		empty = value === void 0;
		if (src) URL.revokeObjectURL(src);
		src = (mimetype || '').startsWith('media/')
			? toObjectURL(JSON.parse(value))
			: '';
	}
</script>

{#if mimetype === 'media/image'}
	<img {src} alt="" />
{:else if mimetype === 'media/video'}
	<video {src}></video>
{:else if mimetype === 'media/audio'}
	<audio {src}></audio>
{:else}
	<pre class:empty><code>{value || '<empty>'}</code></pre>
{/if}

<style>
	.empty {
		user-select: none;
		pointer-events: none;
		-webkit-user-select: none;
		font-style: italic;
		opacity: 0.4;
	}

	img,video,audio {
		max-width: 100%;
	}
</style>
