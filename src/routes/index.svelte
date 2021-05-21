<script context="module">
	export const prerender = true;
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { validate } from 'formee';
	import { dispatch } from '$lib/tauri';
	import { goto } from '$app/navigation';
	import * as Connections from '$lib/stores/connections';

	import Nickname from '$lib/tags/Nickname.svelte';
	import Layout from '$lib/tags/Layout.svelte';
	import Colors from '$lib/tags/Colors.svelte';

	import type { ValidationRules } from 'formee';
	import type { Connection } from '$lib/stores/connections';

	// initial value
	let favorites: Connection[] = [];

	let selected = -1;
	let form: HTMLFormElement;
	let values: Partial<Connection> = {};
	let errors: Partial<Record<keyof Connection, string>> = {};

	$: isEditing = selected > -1;

	const rules: ValidationRules = {
		'nickname': val => {
			if (val == null) return 'Required';
			if (val.length < 2) return 'Must be at least 2 characters';
			return val.length < 24 || 'Must be no more than 24 characters';
		},
		'color': val => {
			if (val == null) return 'Required';
			return /^#([a-f0-9A-F]{3}){1,2}$/.test(val) || 'Must be valid HEX code';
		},
		'host': val => {
			if (val == null) return 'Required';
			return true; // TODO: IP address
		},
		'port': val => {
			if (val == null) return 'Required';
			return (+val > 1000) || 'Must be at least 1000';
		},
		'username': val => {
			if (val == null) return true; // optional
			return val.length > 2 || 'Must be at least 2 characters';
		},
		'password': val => {
			if (val == null) return true; // optional
			return val.length > 1 || 'Must be at least 1 character';
		},
		// TODO: other redis stuff
		'namespaceid': val => {
			if (val == null) return 'Required';
			return val.length > 16 || 'Invalid value'; // TODO
		},
		'accountid': val => {
			if (val == null) return 'Required';
			return val.length > 16 || 'Invalid value'; // TODO
		},
		'accesstoken': val => {
			if (val == null) return 'Required';
			return val.length > 16 || 'Invalid value'; // TODO
		},
	};

	function isValid() {
		errors = validate(form, rules);
		return form.isValid;
	}

	function select(index: number) {
		selected = index;
		values = selected > -1 ? favorites[selected] : {};
	}

	async function toConnect() {
		if (!isValid()) return;

		let redis = {
			host: values.host,
			port: values.port,
			// TODO: others
		};

		console.log('~> connect', redis);
		await dispatch('redis_connect', redis);

		await dispatch('redis_select', {
			namespaceid: values.namespaceid,
		});

		Connections.select(values as Connection);
		form.reset();

		goto('/viewer');
	}

	async function toFavorite() {
		if (!isValid()) return;

		// editing favorite VS is new fav
		if (isEditing) favorites[selected] = values as Connection;
		else favorites.push(values as Connection);
		Connections.update(favorites);
		favorites = favorites;
	}

	// saving changes to favorite
	async function toUpdate() {
		if (!isValid()) return;

		// editing favorite VS is new fav
		favorites[selected] = values as Connection;
		Connections.update(favorites);
		favorites = favorites;
	}

	onMount(() => {
		// read localstorage on DOM ready
		favorites = Connections.refresh();
	});
</script>

<Layout class="connect">
	<svelte:fragment slot="aside">
		<header class:active={!isEditing} on:click={() => select(-1)}>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"/></svg>
			Quick Connect
		</header>

		{#if favorites.length > 0}
			<h4>Favorites</h4>

			<ul>
				{#each favorites as fav,idx (idx)}
					<li
						class="navitem"
						class:active={selected === idx}
						on:click={() => select(idx)}
					>
						<Nickname
							class="fav"
							label={ fav.nickname }
							color={ fav.color }
						/>
					</li>
				{/each}
			</ul>
		{/if}
	</svelte:fragment>

	<form slot="content" bind:this={form} on:submit|preventDefault={toConnect}>
		<div class="input" class:invalid={!!errors.nickname}>
			<label for="c-nickname">Nickname</label>
			<input
				id="c-nickname" name="nickname"
				type="text" bind:value={values.nickname}
			/>
		</div>

		<div class="input colors" class:invalid={!!errors.color}>
			<label for="c-color">Color</label>
			<Colors
				id="c-color" name="color"
				bind:value={values.color}
			/>
		</div>

		<fieldset>
			<legend>Redis Connection</legend>

			<div class="input" class:invalid={!!errors.host}>
				<label for="r-host">Host</label>
				<input
					id="r-host" name="host"
					type="text" bind:value={values.host}
					placeholder="localhost" required
				/>
			</div>

			<div class="input" class:invalid={!!errors.port}>
				<label for="r-port">Port</label>
				<input
					id="r-port" name="port"
					type="number" bind:value={values.port}
					placeholder="6379" required
				/>
			</div>

			<div class="input" class:invalid={!!errors.username}>
				<label for="r-username">Username</label>
				<input
					id="r-username" name="username"
					type="text" bind:value={values.username}
				/>
			</div>

			<div class="input" class:invalid={!!errors.password}>
				<label for="r-password">Password</label>
				<input
					id="r-password" name="password"
					type="password" bind:value={values.password}
				/>
			</div>
		</fieldset>

		<fieldset>
			<legend>Cloudflare Details</legend>

			<div class="input" class:invalid={!!errors.namespaceid}>
				<label for="kv-nsid">Namespace ID</label>
				<input
					id="kv-nsid" name="namespaceid"
					bind:value={values.namespaceid} required
				/>
			</div>

			<div class="input" class:invalid={!!errors.accountid}>
				<label for="kv-acct">Account ID</label>
				<input
					id="kv-acct" name="accountid"
					bind:value={values.accountid} required
				/>
			</div>

			<div class="input" class:invalid={!!errors.accesstoken}>
				<label for="kv-token">Access Token</label>
				<input
					id="kv-token" name="accesstoken"
					type="password" bind:value={values.accesstoken}
					required
				/>
			</div>
		</fieldset>

		<footer>
			{#if isEditing}
				<button type="button" class="white" on:click={toUpdate}>Save Changes</button>
			{:else}
				<button type="button" class="white" on:click={toFavorite}>Add to Favorites</button>
			{/if}

			<button type="submit" class="primary">Connect</button>
		</footer>
	</form>
</Layout>

<style>
	:global(.connect section) {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	form {
		--w: 440px;
		min-width: var(--w);
		padding: 1rem 2rem;
		background-color: #e9ecef;
		width: clamp(var(--w), 50%, 600px);
		border-radius: var(--radius);
		flex-direction: column;
		border: var(--border);
		display: flex;
	}

	fieldset {
		padding: 0.5rem;
		margin: 0.5rem 0;
		border: 1px solid #dee2e6;
		border-radius: var(--radius);
	}

	legend {
		font-weight: 300;
		padding: 0 0.5rem;
	}

	.input {
		display: grid;
		margin: 0.25rem 0;
		grid-template-columns: 6rem 1fr;
		align-items: center;
	}

	.input label {
		font-weight: 600;
		font-size: 0.7rem;
	}

	.input.invalid input {
		border-color: #fa5252;
	}

	header {
		--c: #ffd43b;
		padding: 1rem;
		align-items: center;
		justify-content: flex-end;
		background: var(--bgc, transparent);
		transition: background 200ms linear;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
	}

	header svg {
		width: 1.25rem;
		height: 1.25rem;
		margin-right: 0.375rem;
		transition: fill 200ms linear;
		fill: var(--c);
	}

	header:hover {
		--bgc: #e9ecef;
		--c: #ffa94d;
	}

	header.active {
		--bgc: #e9ecef;
		--c: #fd7e14;
	}

	h4 {
		margin-top: 2rem;
		margin-bottom: 0.25rem;
		padding-left: 1rem;
		font-weight: 300;
	}

	.navitem {
		list-style: none;
		padding-left: 2rem;
		padding-bottom: 0.5rem;
		padding-top: 0.5rem;
	}

	.navitem.active {
		--bgc: #e9ecef;
	}

	.navitem :global(.fav::before) {
		width: 0.65rem;
		margin-right: 0.5rem;
		height: 0.65rem;
	}

	footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: 0.5rem;
	}
</style>
