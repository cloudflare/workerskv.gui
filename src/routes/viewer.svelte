<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	import * as KV from '$lib/utils/kvn';
	import { dispatch } from '$lib/tauri';
	import { timestamp } from '$lib/utils';
	import { active } from '$lib/stores/connections';
	import Layout from '$lib/tags/Layout.svelte';

	let keys: string[] = [];
	let search: HTMLInputElement;

	let pattern = '';
	let sorting = 0; // freeform

	$: isFiltering = pattern.length > 0;
	$: nosorting = keys.length === 0 || isFiltering;

	async function disconnect() {
		await dispatch('redis_disconnect');
		goto('/');
	}

	async function filter(pattern: string) {
		keys = await dispatch<string[]>('redis_filter', { pattern });
	}

	async function synchronize() {
		// NOTE: tauri changes this to a Promise<boolean>
		if (await window.confirm('Are you sure? Will incur charges')) {
			// TODO: pull from $active
			let pager = KV.list(ACCT, NSID, TOKEN);

			let seconds = timestamp();

			for await (let payload of pager) {
				let arr = await Promise.all(
					payload.keys.map(info => {
						return dispatch('redis_set', {
							name: info.name,
							syncd: seconds,
							expires: info.expiration ? String(info.expiration) : null,
							metadata: info.metadata ? JSON.stringify(info.metadata) : null,
						}).then(() => info.name);
					})
				);

				keys = keys.concat(arr);
				if (payload.done) break;
			}

			await dispatch('redis_sync', {
				timestamp: timestamp()
			});
		}
	}

	// TODO: debounce
	async function oninput() {
		pattern = search.value;
		if (pattern.length > 0) {
			await filter(pattern);
		} else {
			await onload();
		}
	}

	async function onsort() {
		if (nosorting) {
			return console.log('not allowed');
		}
		if (++sorting > 2) {
			sorting = 0; // reset -> nosort
			await onload();
		} else {
			let descending = sorting === 2;
			keys = await dispatch<string[]>('redis_sort', { descending });
		}
	}

	async function onload() {
		keys = await dispatch<string[]>('redis_keylist');
	}

	onMount(onload);
</script>

<Layout class="viewer" wider>
	<svelte:fragment slot="aside">
		<header>
			<input
				bind:this={search} on:input={oninput}
				type="search" placeholder="Search keys"
				autocapitalize="off" autocomplete="off"
			/>
		</header>

		<nav>
			<span disabled={nosorting} on:click={onsort}>Key</span>
			<button on:click={synchronize}>SYNC</button>
		</nav>

		{#if keys.length > 0}
			<ul class="keylist">
				{#each keys as keyname (keyname)}
				<li>{keyname}</li>
				{/each}
			</ul>
		{:else}
			<div class="empty-keys">
				{#if isFiltering}
					<span>No keys matched your search filter</span>
				{:else}
					<span>You have not run <pre>SYNC</pre> yet</span>
					<small>OR</small>
					<span>Namespace is empty</span>
				{/if}
			</div>
		{/if}
	</svelte:fragment>

	<div slot="content">
		<header>
			<span>Connected to: <b>{ $active.nickname }</b></span>
			<button on:click={disconnect}>Disconnect</button>
		</header>

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

	:global(.viewer aside) {
		display: flex;
		flex-direction: column;
	}

	:global(.viewer aside) header {
		background: #f1f3f5;
		padding: 0.75rem 1rem;
		display: block;
	}

	:global(.viewer aside) input {
		font-size: 2rem;
		width: 100%;
	}

	:global(.viewer aside ul) {
		overflow-y: auto;
		flex: 1;
	}

	:global(.viewer aside li) {
		border-bottom: var(--border);
	}

	nav {
		display: grid;
		align-items: center;
		grid-template-columns: 1fr 80px;
		font-size: 0.5rem;
		text-transform: uppercase;
		background: #ebebeb;
		font-weight: bold;
		color: #4a4a4a;
		height: 1.5rem;
		width: 100%;
	}

	:global(.viewer aside nav>span),
	:global(.viewer aside li) {
		padding: 0.25rem 0.5rem;
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
