<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { dispatch } from '$lib/tauri';

	import * as utils from '$lib/utils';
	import * as KV from '$lib/utils/kvn';
	import * as mime from '$lib/utils/mime';
	import { active } from '$lib/stores/connections';

	import List from '@sveltejs/svelte-virtual-list';
	import Nickname from '$lib/tags/Nickname.svelte';
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
	$: descending = sorting === 2;
	$: ascending = sorting === 1;

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
		if (!await proceed()) return;

		let { accountid, namespaceid, accesstoken } = $active;
		let pager = KV.list(accountid, namespaceid, accesstoken);
		let seconds = utils.timestamp();

		let keybatch: string[] = [];

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

			keybatch = keybatch.concat(arr);
			if (payload.done) break;
		}

		// client update
		keylist = keybatch;
		lastsync = utils.timestamp();

		await dispatch('redis_sync', {
			timestamp: lastsync
		});
	}

	function isTag(tag: 'video' | 'img' | 'audio', src: string): Promise<boolean> {
		return new Promise(res => {
			let elem = document.createElement(tag);
			elem.onerror = () => res(false);
			elem.onload = () => res(true);
			elem.src = src;
		});
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
		let { accountid, namespaceid, accesstoken } = $active;
		let value = await KV.retrieve(accountid, namespaceid, accesstoken, name);

		console.log({ value, seconds });

		// TODO: real mimetype sniffing
		let info = await mime.parse(value);

		await dispatch('redis_value', {
			key: name,
			value: info.value,
			mimetype: info.mimetype,
			timestamp: seconds,
		});

		// client-side updates
		details.lastupdate = seconds;
		details.mimetype = info.mimetype;
		details.value = info.value;
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

		<nav class="keynav" class:descending class:ascending>
			<span disabled={nosorting} on:click={onsort}>
				Key
				<svg class="i-asc" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path d="M3 3a1 1 0 000 2h11a1 1 0 100-2H3zm0 4a1 1 0 000 2h5a1 1 0 000-2H3zm0 4a1 1 0 100 2h4a1 1 0 100-2H3zm10 5a1 1 0 102 0v-5.586l1.293 1.293a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 101.414 1.414L13 10.414V16z"/></svg>
				<svg class="i-desc" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path d="M3 3a1 1 0 000 2h11a1 1 0 100-2H3zm0 4a1 1 0 000 2h7a1 1 0 100-2H3zm0 4a1 1 0 100 2h4a1 1 0 100-2H3zm12-3a1 1 0 10-2 0v5.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L15 13.586V8z"/></svg>
			</span>
			<button class="primary sm" on:click={synchronize}>SYNC</button>
		</nav>

		{#if keylist.length > 0}
			<List items={keylist} let:item>
				<span
					class="navitem keyitem"
					class:selected={item === viewing}
					on:click={onexpand}>{item}</span>
			</List>
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
		<header>
			<small>
				Last Sync: <Date value={lastsync} />
			</small>

			<span class="connection">
				{#if $active.nickname}
					<Nickname
						label={ $active.nickname }
						color={ $active.color }
					/>
				{:else}
					<Nickname
						label="{ $active.host }:{ $active.port }"
					/>
				{/if}
				<button class="sm" on:click={disconnect}>Disconnect</button>
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
					<Value
						value={details.expires && utils.date(details.expires)}
					/>
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
						<small>
							Last Update:
							<Date value={details.lastupdate}/>
						</small>
					</span>
					<button class="sm" on:click={retrieve}>Refresh</button>
				</div>
				<span class="value key-value">
					<Value
						value={details.value}
						mimetype={details.mimetype}
					/>
				</span>
			</div>

			{#if !details.name}
				<div class="fields-empty">
					<p>Please select a KEY from the sidebar</p>
				</div>
			{/if}
		</div>
	</div>
</Layout>

<style>
	:global(.viewer aside) header {
		padding: 0.5rem;
		background: #f1f3f5;
		display: block;
	}

	:global(.viewer aside) input {
		height: 2rem;
		width: 100%;
	}

	:global(.viewer aside) input:not(:placeholder-shown),
	:global(.viewer aside) input:hover {
		border-color: #ffd8a8;
	}

	:global(.viewer aside svelte-virtual-list-viewport) {
		overflow-y: auto;
		flex: 1;
	}

	.keyitem {
		font-family: menlo, inconsolata, monospace;
		border-left: 4px solid transparent;
		border-bottom: var(--border);
	}

	.keyitem.selected {
		border-left-color: #fd7e14;
	}

	.keynav {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: 0.5rem;
		text-transform: uppercase;
		background: #ebebeb;
		font-weight: bold;
		color: #4a4a4a;
		height: 1.75rem;
		width: 100%;
	}

	.keynav > span {
		padding: 0.375rem 0.5rem;
		-webkit-user-select: none;
		user-select: none;
		display: inline-flex;
		align-items: center;
		cursor: pointer;
		height: 100%;
		width: 100%;
	}

	.keynav svg {
		color: #343a40;
		position: absolute;
		visibility: hidden;
		height: 0.7rem;
		width: 0.7rem;
		left: 2rem;
	}

	.keynav.descending .i-desc {
		visibility: visible;
	}

	.keynav.ascending .i-asc {
		visibility: visible;
	}

	/* SYNC button */
	.keynav button {
		margin: 0 0.5rem;
		font-size: 0.55rem;
		padding-right: 1rem;
		padding-left: 1rem;
		font-weight: bold;
		height: 1.25rem;
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
		border-radius: var(--radius);
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
		text-transform: none;
		font-weight: 600;
	}

	.connection {
		display: flex;
		align-items: center;
	}

	/* DISCONNECT BTN */
	.connection button {
		--c: #fff;
		--bgc: #f03e3e;
		--box: #f03e3e;
		font-size: 0.55rem;
		margin-left: 0.5rem;
		font-weight: 500;
	}

	.fields {
		display: grid;
		grid-template-columns: 1fr 1fr;
		grid-auto-rows: min-content;
		height: calc(100vh - 3rem);
		position: relative;
		overflow-y: auto;
	}

	.fields.disabled {
		cursor: not-allowed;
	}

	.fields.disabled,
	.fields-empty {
		pointer-events: none;
		-webkit-user-select: none;
		user-select: none;
	}

	.fields-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(52, 58, 64, 0.7);
		position: absolute;
		color: #f1f3f5;
		font-weight: 300;
		bottom: 0;
		right: 0;
		left: 0;
		top: 0;
	}

	.cell {
		padding: 0.5rem;
		grid-column: span 2;
	}

	.cell.w50 {
		grid-column: span 1;
	}

	.cell .label {
		font-size: 10px;
		letter-spacing: 0.2px;
		text-transform: uppercase;
		font-weight: bold;
	}

	.cell .value {
		display: block;
		font-size: 0.85rem;
		padding: 0.25rem 0.5rem;
		border-radius: var(--radius);
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
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.25rem;
	}

	.keyvalue .label {
		width: 100%;
	}

	.keyvalue small {
		display: block;
		font-size: 0.5rem;
	}

	/* REFRESH BTN */
	.keyvalue button {
		--c: #fff;
		/* --bgc: #f76707;
		--box: #d9480f; */
		--bgc: #15aabf;
		--box: #1098ad;
		font-size: 0.5rem;
		text-transform: uppercase;
		font-weight: 600;
		height: 1.25rem;
	}
</style>
