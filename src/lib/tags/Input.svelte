<script>
	export let value = null;
	export let type = 'text';
	export let required = false;
	export let label, name, id, placeholder;
</script>

<div class="input">
	<label for={id}>{label}</label>
	{#if type === 'text'}
		<input {id} type="text" bind:value {name} {required} {placeholder} />
	{:else if type === 'number'}
		<input {id} type="number" bind:value {name} {required} {placeholder} />
	{:else if type === 'color'}
		<input {id} type="color" bind:value {name} {required} {placeholder} />
	{:else if type === 'password'}
		<input {id} type="password" bind:value {name} {required} {placeholder} />
	{/if}
</div>
