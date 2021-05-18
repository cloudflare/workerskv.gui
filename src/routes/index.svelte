<script lang="ts">
	import { dispatch } from '$lib/tauri';

	let form: HTMLFormElement;

	function run(cmd: string, arg?: any): EventListener {
		return async function (ev) {
			console.log('inside', cmd, arg);
			let output = await dispatch(cmd, arg);
			console.log('RECEIVED', output);
		};
	}

	async function onconnect(ev: Event) {
		let fdata = new FormData(form);
		let values = Object.fromEntries(fdata);

		await dispatch('redis_connect', {
			host: values.host as string,
			port: +values.port as number,
		});
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={run('hello')}>hello</button>
<button on:click={run('simple_command', {argument:'hiya'})}>simple_command</button>
<button on:click={run('stateful_command')}>stateful_command</button>
<button on:click={run('window_label')}>window_label</button>

<hr>

<h2>Redis Connection</h2>
<form bind:this={form} on:submit|preventDefault={onconnect}>
	<label for="redis:host">Host</label>
	<input id="redis:host" type="text" name="host" placeholder="localhost" required>

	<label for="redis:port">Port</label>
	<input id="redis:port" type="number" name="port" placeholder="6379" required>

	<label for="redis:password">Password</label>
	<input id="redis:password" type="password" name="password">

	<label for="redis:ssl">
		<input type="checkbox" id="redis:ssl" name="ssl">
		SSL
	</label>

	<button type="submit">Connect</button>
</form>

<style>
	button, label {
		display: block;
	}
</style>
