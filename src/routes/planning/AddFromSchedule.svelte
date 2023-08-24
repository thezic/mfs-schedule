<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Button, Dropdown, Input } from 'flowbite-svelte';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { ChevronDown } from '@steeze-ui/heroicons';
	import { MinistryEventTemplate } from '$lib/models/MinistryEvent';

	export let week: Date;
	let _week = week;

	const dispatch = createEventDispatcher<{ createFromTemplate: MinistryEventTemplate }>();

	let templates = [
		new MinistryEventTemplate(1, 'Only weekend', [
			{ day: 6, time: '10:30:00', place: 'Pie astrīdas, Puķu iela 5', extraInfo: '' },
			{ day: 7, place: 'grupas', extraInfo: '' }
		]),
		new MinistryEventTemplate(2, 'week', [
			{ day: 3, time: '18:00:00', place: 'Valstibas zale', extraInfo: '' },
			{ day: 5, time: '10:00:00', place: 'Valstibas zale', extraInfo: '' },
			{ day: 6, time: '10:30:00', place: 'Valstibas zale', extraInfo: '' },
			{ day: 7, place: 'grupas', extraInfo: '' }
		])
	];

	function onAdd(template: MinistryEventTemplate) {
		dispatch('createFromTemplate', template);
	}
</script>

<Button>Add from schedule <Icon class="w-4 h-4 ml-2" src={ChevronDown} /></Button>
<Dropdown>
	<div class="p-4">
		<ul class="divide-y">
			{#each templates as template (template.id)}
				<li class="flex justify-between py-2">
					{template.name}
					<button
						class="uppercase px-2 text-primary-700 rounded hover:bg-primary-600 hover:text-white"
						on:click={() => onAdd(template)}>Add</button
					>
				</li>
			{/each}
		</ul>
		<Input bind:value={_week} />
	</div>
</Dropdown>
