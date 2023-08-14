<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { ButtonGroup, Input } from 'flowbite-svelte';
	import { formatDate } from '$lib/utils/date';

	// export let value: Date;
	export let date: Date;
	export let time: string | undefined;

	const dispatch = createEventDispatcher();

	function change() {
		console.log({ date, time });
		dispatch('change', { date, time });
	}

	let _date = formatDate(date);
	let _time = time ?? '';

	$: date = new Date(Date.parse(_date));
	$: {
		const parts = _time.split(':');
		if (parts.length > 1) {
			parts[2] = '00';
			time = parts?.join(':');
		} else {
			time = undefined;
		}

		console.log(time);
	}
</script>

<div class="flex gap-2 w-min">
	<ButtonGroup class="flex-1" size="xs">
		<Input bind:value={_date} type="date" class="w-min" size="sm" on:change={change} />
		<Input bind:value={_time} type="time" class="w-min" size="sm" on:change={change} />
	</ButtonGroup>
</div>
