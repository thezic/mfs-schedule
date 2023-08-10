<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { ButtonGroup, Input } from 'flowbite-svelte';

	export let value: Date;

	const dispatch = createEventDispatcher();

	function change() {
		dispatch('change', value);
	}

	function pad(n: number, padding = 2): string {
		return n.toString().padStart(padding, '0');
	}

	let date = `${value.getFullYear()}-${pad(value.getMonth() + 1)}-${pad(value.getDate())}`;
	let time = `${pad(value.getHours())}:${pad(value.getMinutes())}`;

	$: value = new Date(Date.parse(`${date}T${time}`));
</script>

<div class="flex gap-2 w-min">
	<ButtonGroup class="flex-1" size="xs">
		<Input bind:value={date} type="date" class="w-min" size="sm" on:change={change} />
		<Input bind:value={time} type="time" class="w-min" size="sm" on:change={change} />
	</ButtonGroup>
</div>
