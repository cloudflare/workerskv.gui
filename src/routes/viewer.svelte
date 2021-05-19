<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	import { dispatch, sync } from '$lib/tauri';
	import { active } from '$lib/stores/connections';
	import Layout from '$lib/tags/Layout.svelte';

	import type { Key } from '$lib/tauri';

	let keys: Key[] = [];
	let search: HTMLInputElement;

	async function disconnect() {
		await dispatch('redis_disconnect');
		goto('/');
	}

	async function filter(pattern: string) {
		let output = await dispatch('redis_filter', { pattern });
		console.log('> output', output);
		// keys = ...
	}

	async function oninput(ev) {
		console.log('inside', search.value);
		if (search.value.length > 0) {
			await filter(search.value);
		} else {
			await onload();
		}
	}

	async function onload() {
		console.log('INSIDE ONLOAD');
		let output = await dispatch('redis_load');
		console.log(output);
	}

	onMount(onload);
</script>

<Layout class="viewer" wider>
	<svelte:fragment slot="aside">
		<header>
			<input
				type="search"
				bind:this={search}
				placeholder="Search keys"
				on:input={oninput}
			/>
		</header>

		<nav class="grid">
			<span>Key</span>
			<span>Last Sync</span>
			<!-- <th>Metadata</th> -->
		</nav>

		{#if keys.length > 0}
			<ul class="grid">
				{#each keys as key (key.name)}
				<li>

				</li>
				{/each}
			</ul>
		{:else}
			<div class="empty-keys">
				<span>You have not run <pre>SYNC</pre> yet</span>
				<small>OR</small>
				<span>Namespace is empty</span>
			</div>
		{/if}
	</svelte:fragment>

	<div slot="content">
		<header>
			<span>Connected to: <b>{ $active.nickname }</b></span>
			<button on:click={disconnect}>Disconnect</button>
		</header>

		<a href="/">Home</a>
		<pre>
			{ JSON.stringify($active, null, 2) }
		</pre>
	</div>
</Layout>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		border-bottom: var(--border);
		height: 3rem;
	}

	:global(.viewer aside) header {
		background: #f1f3f5;
		padding: 0.75rem 1rem;
	}

	:global(.viewer aside) input {
		flex: 1;
		font-size: 2rem;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(25%, 75%));
	}

	nav.grid {
		height: 1.5rem;
		font-weight: bold;
		text-transform: uppercase;
		align-items: center;
		font-size: 0.5rem;
		background: #ebebeb;
		color: #4a4a4a;
	}

	:global(.viewer aside nav>span),
	:global(.viewer aside li>span) {
		border-right: 1px solid #c3c3c3;
		padding: 0.25rem 0.5rem;
	}

	:global(.viewer aside nav>span:last-of-type),
	:global(.viewer aside li>span:last-of-type) {
		border-right: none;
	}

	:global(.viewer section) header {
		background: #dee2e6;
		padding: 0.5rem;
	}

	.empty-keys {
		display: flex;
		flex-direction: column;
		align-items: center;
		font-size: 0.8rem;
		padding: 4rem 0;
		color: #495057;
	}

	.empty-keys pre {
		display: inline;
		background: #dee2e6;
		border-radius: 4px;
		font-size: 75%;
		padding: 0.5em;
	}

	.empty-keys small {
		margin: 0.25rem 0;
		font-size: 0.6rem;
	}
</style>
