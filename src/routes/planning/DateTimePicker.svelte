<script lang="ts">
	import { ButtonGroup, Input, Button, InputAddon } from 'flowbite-svelte';
	import { Calendar, Clock } from '@steeze-ui/heroicons';
	import { Icon } from '@steeze-ui/svelte-icon';

	export let value: Date;

	function pad(n: number, padding = 2): string {
		return n.toString().padStart(padding, '0');
	}

	let date = `${value.getFullYear()}-${pad(value.getMonth() + 1)}-${pad(value.getDate())}`;
	let time = `${pad(value.getHours())}:${pad(value.getMinutes())}`;

	$: value = new Date(Date.parse(`${date}T${time}`));
</script>

<div class="flex gap-2 w-min">
	<ButtonGroup class="flex-1" size="xs">
		<Input bind:value={date} type="date" class="w-min" size="sm" />
		<!--<InputAddon><Icon class="w-6 h-6" src={Calendar} /></InputAddon>-->
		<Input bind:value={time} type="time" class="w-min" size="sm" />
		<!--<InputAddon><Icon class="w-6 h-6" src={Clock} /></InputAddon>-->
	</ButtonGroup>
</div>
