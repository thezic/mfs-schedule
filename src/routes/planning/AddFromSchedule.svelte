<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Button, Dropdown, Input } from 'flowbite-svelte';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { ChevronDown } from '@steeze-ui/heroicons';
	import { MinistryEventTemplate } from '$lib/models/MinistryEvent';
	import { formatDate } from '$lib/utils/date';

	export let week: Date;
	let _week = formatDate(week);

	$: _week = formatDate(week);

	const dispatch = createEventDispatcher<{ createFromTemplate: MinistryEventTemplate }>();

	let templates = load();

	function load(): MinistryEventTemplate[] {
		const json = localStorage.getItem('TEMPLATES');
		const data = json ? JSON.parse(json) : [];
		return data.map((t: any) => new MinistryEventTemplate(t.id, t.name, t.template));
	}

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
