<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { dispatch } from '$lib/tauri';

	import * as utils from '$lib/utils';
	import * as KV from '$lib/utils/kvn';
	import { active } from '$lib/stores/connections';

	import Layout from '$lib/tags/Layout.svelte';
	import Value from '$lib/tags/Value.svelte';
	import Date from '$lib/tags/Date.svelte';

	import type { Key } from '$lib/utils/kvn';

	let lastsync = '';
	let keylist: string[] = [];

	let search: HTMLInputElement;
	let details: Partial<Key> = {};

	let pattern = '';
	let sorting = 0; // freeform
	let viewing = ''; // keyname -> details

	let ack_value = false; // proceed() for charge

	$: isFiltering = pattern.length > 0;
	$: nosorting = keylist.length === 0 || isFiltering;

	async function disconnect() {
		await dispatch('redis_disconnect');
		goto('/');
	}

	async function filter(pattern: string) {
		keylist = await dispatch<string[]>('redis_filter', { pattern });
	}

	async function proceed(): Promise<boolean> {
		// NOTE: tauri changes this to a Promise<boolean>
		return window.confirm('Are you sure? Will incur charges');
	}

	async function synchronize() {
		if (await proceed()) {
			// TODO: pull from $active
			let pager = KV.list(ACCT, NSID, TOKEN);

			let seconds = utils.timestamp();

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

				keylist = keylist.concat(arr);
				if (payload.done) break;
			}

			await dispatch('redis_sync', {
				timestamp: utils.timestamp()
			});
		}
	}

	/**
	 * get the value for a single key
	 */
	async function retrieve() {
		// Will cost $$ to read a KV value
		if (!ack_value && !await proceed()) return
		ack_value = true; // dont ask anymore

		let name = details.name;
		let seconds = utils.timestamp();
		let value = await KV.retrieve(ACCT, NSID, TOKEN, name);
		console.log({ value, seconds });

		await dispatch('redis_value', {
			key: name,
			value: value,
			timestamp: seconds,
			mimetype: null, // TODO
		});
		// client-side updates
		details.lastupdate = seconds;
		details.mimetype = null;
		details.value = value;
		details = details;
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

	async function onexpand(ev: Event) {
		let key = (ev.target as HTMLElement).innerText;
		details = await dispatch<Partial<Key>>('redis_details', { key });
		details.name = viewing = key;
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
			keylist = await dispatch<string[]>('redis_sort', { descending });
		}
	}

	async function onload() {
		keylist = await dispatch<string[]>('redis_keylist');
		if (!lastsync) lastsync = await dispatch<string>('redis_lastsync');
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

		{#if keylist.length > 0}
			<ul class="keylist">
				{#each keylist as keyname (keyname)}
				<li class:selected={keyname === viewing} on:click={onexpand}>{keyname}</li>
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

	<div class="details" slot="content">
		<header style="--c: {$active.color}">
			<small>
				Last Sync: <Date value={lastsync} />
			</small>

			<span>
				{#if $active.nickname}
					<b>{ $active.nickname }</b>
				{:else}
					<b>{ $active.host }:{ $active.port }</b>
				{/if}
				<button on:click={disconnect}>Disconnect</button>
			</span>
		</header>

		<div class="fields" class:disabled={!details.name}>
			<div class="cell">
				<span class="label">Name</span>
				<span class="value">
					<pre><code>{details.name}</code></pre>
				</span>
			</div>

			<div class="cell w50">
				<span class="label">Last Seen</span>
				<span class="value">
					<Value value={details.syncd ? utils.date(details.syncd) : undefined} />
				</span>
			</div>

			<div class="cell w50">
				<span class="label">Expiration</span>
				<span class="value">
					<Value value={details.expires} />
				</span>
			</div>

			<div class="cell">
				<span class="label">Metadata</span>
				<span class="value key-metadata">
					<Value value={details.metadata} />
				</span>
			</div>

			<div class="cell keyvalue">
				<div>
					<span class="label">
						Value
						{#if details.lastupdate}
							<small><Date value={details.lastupdate}/></small>
						{/if}
					</span>
					<button>Refresh</button>
				</div>
				<span class="value key-value">
					<Value value={details.value} />
				</span>
			</div>
		</div>

		<pre>
			{ JSON.stringify(details, null, 2) }
		</pre>

		<button on:click={retrieve}>Retrieve Value</button>
		<!-- {#if details.value == null}
		{/if} -->
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

	.details header {
		background: #e9ecef;
		font-size: 0.8rem;
		padding: 0.5rem;
	}

	.details small {
		font-size: 0.6rem;
		font-style: italic;
		font-weight: 600;
	}

	.details b {
		margin-right: 0.5rem;
	}

	.details b::before {
		content: '';
		position: relative;
		display: inline-block;
		margin-right: 0.25rem;
		background: var(--c, transparent);
		border-radius: 50%;
		height: 0.5rem;
		width: 0.5rem;
	}

	.fields {
		display: grid;
		grid-template-columns: 1fr 1fr;
		grid-auto-rows: min-content;
		height: calc(100vh - 3rem);
		overflow-y: auto;
	}

	.cell {
		padding: 0.5rem;
		grid-column: span 2;
	}

	.cell.w50 {
		grid-column: span 1;
	}

	.cell .label {
		font-size: 8px;
		text-transform: uppercase;
		font-weight: bold;
	}

	.cell .value {
		display: block;
		font-size: 0.85rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		background: #dee2e6;
		overflow-x: auto;
	}

	.key-metadata :global(pre:not(.empty)) {
		min-height: 4rem;
	}

	.key-value :global(pre) {
		min-height: 6rem;
	}

	.keyvalue > div {
		display: flex;
		justify-content: space-between;
	}
</style>
