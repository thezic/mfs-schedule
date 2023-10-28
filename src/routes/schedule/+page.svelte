<script lang="ts">
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import { MinistryEventTemplate, type UndatedEvent } from '$lib/models/MinistryEvent';
	import { Button } from 'flowbite-svelte';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { Trash } from '@steeze-ui/heroicons';

	let templates = load();

	function addNewEvent(template: MinistryEventTemplate) {
		const day = Math.max(...template.template.map((ev) => ev.day));
		template.add({ day, time: undefined, extraInfo: '', place: '' });
		templates = templates;
	}

	function addTemplate() {
		const id = Math.max(...templates.map((t) => t.id)) + 1;
		templates.push(new MinistryEventTemplate(id, 'New template', []));

		templates = templates;
	}

	function sorted(events: UndatedEvent[]) {
		const copy = events.slice();
		copy.sort((a, b) => a.day - b.day);
		return copy;
	}

	function deleteEvent(template: MinistryEventTemplate, event: UndatedEvent) {
		template.remove(event);
		templates = templates;
	}

	function duplicateTemplate(template: MinistryEventTemplate) {
		const id = Math.max(...templates.map((t) => t.id)) + 1;
		const name = template.name + '-copy';
		const events_string = JSON.stringify(template.template);
		const events = JSON.parse(events_string);
		const newTemplate = new MinistryEventTemplate(id, name, events);
		templates.push(newTemplate);
		templates = templates;
	}

	function deleteTemplate(template: MinistryEventTemplate) {
		templates = templates.filter((t) => t !== template);
	}

	const save = (function () {
		let handle: number | undefined;
		return (templates: MinistryEventTemplate[]) => {
			if (handle) {
				clearTimeout(handle);
			}
			handle = setTimeout(() => {
				let json = JSON.stringify(templates);
				localStorage.setItem('TEMPLATES', json);
				console.log('Save');
			}, 500) as unknown as number;
		};
	})();

	function load(): MinistryEventTemplate[] {
		const json = localStorage.getItem('TEMPLATES');
		const data = json ? JSON.parse(json) : [];
		return data.map((t: any) => new MinistryEventTemplate(t.id, t.name, t.template));
	}

	$: save(templates);
</script>

<MainLayout>
	<h1 slot="header-left" class="text-base font-semibold leading-6 text-gray-900">Schedules</h1>
	<div slot="header-right"><Button on:click={() => addTemplate()}>Add template</Button></div>
	<div class="h-full overflow-auto">
		<vbox class="gap-2 divide-y">
			{#each templates as template (template.id)}
				<vbox>
					<hbox class="bg-sky-700 font-bold p-2 justify-between"
						><input
							bind:value={template.name}
							class="px-2 py-1 border border-gray-400 rounded"
						/><hbox />
						<hbox class="gap-2">
							<Button on:click={() => duplicateTemplate(template)}>Duplicate</Button>
							<Button on:click={() => deleteTemplate(template)}>Delete</Button>
						</hbox>
					</hbox>
					<table>
						<thead class="contents">
							<tr>
								<th>Day</th>
								<th>Time</th>
								<th>Place</th>
								<th>Extra Info</th>
								<th />
							</tr>
						</thead>
						<tbody class="contents">
							{#each sorted(template.template) as eventTemplate}
								<tr>
									<td>
										<select bind:value={eventTemplate.day}>
											<option value={1}>Monday</option>
											<option value={2}>Tuesday</option>
											<option value={3}>Wednesday</option>
											<option value={4}>Thursday</option>
											<option value={5}>Friday</option>
											<option value={6}>Saturday</option>
											<option value={7}>Sunday</option>
										</select>
									</td>
									<td
										><input
											class="border border-gray-300 rounded px-2 py-1"
											bind:value={eventTemplate.time}
										/></td
									>
									<td
										><input
											class="border border-gray-300 rounded px-2 py-1"
											bind:value={eventTemplate.place}
										/></td
									>
									<td
										><input
											class="border border-gray-300 rounded px-2 py-1"
											bind:value={eventTemplate.extraInfo}
										/></td
									>
									<td>
										<Button
											outline
											size="xs"
											pill
											class="!p-2 border-transparent"
											on:click={() => deleteEvent(template, eventTemplate)}
										>
											<Icon class="h-4 w-4 shrink-0" src={Trash} />
										</Button>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
					<hbox><Button on:click={() => addNewEvent(template)}>ADD</Button> </hbox>
				</vbox>
			{/each}
		</vbox>
	</div>
</MainLayout>

<style lang="postcss">
	vbox {
		@apply flex flex-col;
	}

	hbox {
		@apply flex flex-row;
	}
</style>
