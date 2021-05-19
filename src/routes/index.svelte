<script context="module">
	export const prerender = true;
</script>

<script lang="ts">
	import { validate } from 'formee';
	import { onMount, tick } from 'svelte';
	import * as Connections from '$lib/stores/connections';
	import { goto } from '$app/navigation';

	import Layout from '$lib/tags/Layout.svelte';
	import Input from '$lib/tags/Input.svelte';
	import { dispatch } from '$lib/tauri';

	import type { ValidationRules } from 'formee';

	// initial value
	let favorites = [];

	let selected = -1;
	let form: HTMLFormElement, errors={};
	let values: Partial<Connections.Connection> = {};

	const rules: ValidationRules = {
		'nickname': val => {
			if (val == null) return 'Required';
			if (val.length < 2) return 'Must be at least 2 characters';
			return val.length < 17 || 'Must be no more than 16 characters';
		},
		'color': val => {
			if (val == null) return 'Required';
			return /^#([a-f0-9A-F]{3}){1,2}$/.test(val) || 'Must be valid HEX code';
		},
		'redis.host': val => {
			if (val == null) return 'Required';
			return true; // TODO: IP address
		},
		'redis.port': val => {
			if (val == null) return 'Required';
			return (+val > 1000) || 'Must be at least 1000';
		},
		'redis.username': val => {
			if (val == null) return true; // optional
			return val.length > 2 || 'Must be at least 2 characters';
		},
		'redis.username': val => {
			if (val == null) return true; // optional
			return val.length > 2 || 'Must be at least 2 characters';
		},
		// TODO: others
	};

	function isValid() {
		errors = validate(form, rules);
		return form.isValid;
	}

	$: console.log('errors', errors);

	// function run(cmd: string, arg?: any): EventListener {
	// 	return async function (ev) {
	// 		console.log('inside', cmd, arg);
	// 		let output = await dispatch(cmd, arg);
	// 		console.log('RECEIVED', output);
	// 	};
	// }

	function select(index: number) {
		selected = index;
		values = favorites[selected];
	}

	async function quickconnect(index: number) {
		select(index);
		await tick();
		form.submit();
	}

	async function toConnect(ev: Event) {
		if (!isValid()) return;
		let redis = {
			host: values.host,
			port: values.port,
			// TODO: others
		};
		console.log('~> connect', redis);
		let out = await dispatch('redis_connect', redis);
		console.log('output', out);

		Connections.select(values);
		form.reset();

		goto('/viewer');
	}

	async function toFavorite(ev: Event) {
		if (!isValid()) return;

		// editing favorite VS is new fav
		if (selected === -1) favorites.push(values);
		else favorites[selected] = values;

		favorites = favorites;
	}

	onMount(() => {
		// read localstorage on DOM ready
		favorites = Connections.refresh();
	});
</script>

<Layout class="connect">
	<svelte:fragment slot="aside">
		<header
			class:active={selected === -1}
			on:click={() => select(-1)}
		>
			Quick Connect
		</header>

		{#if favorites.length > 0}
			<h2>FAVORITES</h2>

			<ul>
				{#each favorites as fav,idx (idx)}
					<li
						on:click={() => select(idx)}
						on:dblclick={() => quickconnect(idx)}
					>
						{ fav.nickname } ({fav.color})
					</li>
				{/each}
			</ul>
		{/if}
	</svelte:fragment>

	<form slot="content" bind:this={form} on:submit|preventDefault={toConnect}>
		<Input
			id="c-nickname" name="nickname"
			label="Nickname" bind:value={values.nickname}
		/>

		<Input
			id="c-color" name="color" type="color"
			label="Color" bind:value={values.color}
		/>

		<fieldset>
			<legend>Redis Connection</legend>

			<Input
				label="Host"
				id="r-host" name="redis.host"
				type="text" bind:value={values.host}
				placeholder="localhost" required
			/>

			<Input
				label="Port"
				id="r-port" name="redis.port"
				type="number" bind:value={values.port}
				placeholder="6379" required
			/>

			<Input
				label="Username"
				id="r-username" name="redis.username"
				type="text" bind:value={values.username}
			/>

			<Input
				label="Password"
				id="r-password" name="redis.password"
				type="password" bind:value={values.password}
			/>
		</fieldset>

		<span>
			<button on:click|preventDefault={toFavorite}>
				Add to Favorites
			</button>
			<button type="submit">Connect</button>
		</span>

	</form>
</Layout>

<style>
	:global(.connect section) {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	form {
		background-color: #e9ecef;
		width: clamp(360px, 50%, 600px);
		padding: 1rem 2rem;
	}
</style>
