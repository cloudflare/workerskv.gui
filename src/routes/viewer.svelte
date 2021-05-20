<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	import { dispatch } from '$lib/tauri';
	import { list } from '$lib/utils/kvn';
	import { active } from '$lib/stores/connections';
	import Layout from '$lib/tags/Layout.svelte';

	let keys: string[] = [];
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

	async function synchronize() {
		// NOTE: tauri changes this to a Promise<boolean>
		if (await window.confirm('Are you sure? Will incur charges')) {
			// TODO: pull from $active
			let pager = list(
				'df42c1de9846e351abbfdf885dde761b',
				'802e0f5c830c45d09fcae3f506579341',
				'token1234'
			);

			let seconds = String(Date.now() / 1e3 | 0);

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
				timestamp: String(Date.now() / 1e3 | 0)
			});
		}
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
		keys = await dispatch<string[]>('redis_keylist');
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

		<nav>
			<span>Key</span>
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
