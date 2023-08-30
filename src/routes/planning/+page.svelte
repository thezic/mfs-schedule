<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import getISOWeek from 'date-fns/getISOWeek';
	import type { PageData } from './$types';
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import { Trash } from '@steeze-ui/heroicons';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { compose, sort, groupBy, last, prop, defaultTo, map } from 'rambda';
	import startOfWeek from 'date-fns/startOfWeek';
	import addDays from 'date-fns/addDays';
	import nextMonday from 'date-fns/nextMonday';

	import {
		createEvent as createMinistryEvent,
		deleteEvent as deleteMinistryEvent,
		updateEvent,
		type NewMinistryEvent
	} from 'bindings';

	import DateTimePicker from './DateTimePicker.svelte';
	import PersonSelect from './PersonSelect.svelte';
	import Print from './Print.svelte';

	import { MinistryEvent, MinistryEventTemplate } from '$lib/models/MinistryEvent';
	import { Person } from '$lib/models/Person';
	import { formatDate } from '$lib/utils/date';
	import AddFromSchedule from './AddFromSchedule.svelte';

	export let data: PageData;
	let events = data.events.map((event) => new MinistryEvent(event));
	let persons = data.persons.map((person) => new Person(person));

	const groupByMonth = groupBy((event: MinistryEvent) => {
		const monday = startOfWeek(event.date, { weekStartsOn: 1 });
		const thursday = addDays(monday, 3);
		return '' + ((thursday.getFullYear() << 4) + thursday.getMonth());
	});

	const sortByDate = sort(
		(a: MinistryEvent, b: MinistryEvent) => a.date.getTime() - b.date.getTime()
	);

	async function createEvent() {
		const lastEvent = events.length ? sortByDate(events)[events.length - 1] : undefined;
		const date = lastEvent ? formatDate(lastEvent.date) : formatDate(new Date(Date.now()));

		const newEvent = await createMinistryEvent({
			place: '',
			date,
			time: null,
			extraInfo: '',
			assigneeName: '',
			assigneeId: null
		});

		events.push(new MinistryEvent(newEvent));
		events = events;
	}

	async function createEventsFromTemplate(template: MinistryEventTemplate) {
		const promises = compose(
			map(async (templ: NewMinistryEvent): Promise<MinistryEvent> => {
				console.log(templ);
				const e = await createMinistryEvent(templ);
				return new MinistryEvent(e);
			})
		)(Array.from(template.atWeek(nextWeek)));

		const newEvents = await Promise.all(promises);
		newEvents.forEach((e) => events.push(e));

		events = events;
	}

	async function deleteEvent(eventToDelete: MinistryEvent) {
		await deleteMinistryEvent(eventToDelete.id);

		events = events.filter((event) => event.id !== eventToDelete.id);
	}

	async function save(event: MinistryEvent) {
		try {
			console.log('Saving', event);
			const updatedData = await updateEvent(event.asDto());
			events.splice(events.indexOf(event), 1, new MinistryEvent(updatedData));
			events = events;
		} catch (error) {
			console.error(error);
		}
	}

	const locale = navigator.language;

	function getMonthNameFromKey(month: string) {
		// const m = ['Jan', 'Feb', 'Mar', 'Apr', 'Maj', 'Jun', 'Jul', 'Aug', 'Sep', 'Okt', 'Nov', 'Dec'];
		const date = new Date();
		date.setMonth(+month & 0xf);
		date.setDate(1);
		return date.toLocaleString(locale, { month: 'long' });
		// return m[+month & 0xf];
	}
	// let tableData: { event: MinistryEvent; markMonth: boolean }[];
	$: tableData = compose(groupByMonth, sortByDate)(events);
	$: months = Object.keys(tableData);
	$: nextWeek = compose(nextMonday, defaultTo(new Date()), prop('date'), last, sortByDate)(events);
</script>

<MainLayout>
	<div slot="header-right" class="flex justify-between gap-4">
		<AddFromSchedule
			week={nextWeek}
			on:createFromTemplate={(e) => createEventsFromTemplate(e.detail)}
		/>
		<Button on:click={createEvent}>Add meeting</Button>
		<Print />
	</div>
	<div class="h-full overflow-auto">
		<table
			class="min-w-full grid grid-cols-[min-content,min-content,min-content,minmax(min-content,13rem),repeat(2,1fr),min-content]"
		>
			<thead class="contents">
				<tr class="contents">
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2">Wk</th>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2">Time</th
					>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2">Day</th>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2"
						>Assignee</th
					>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2"
						>Place</th
					>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2"
						>Extra info</th
					>
					<th class="sticky top-0 h-7 z-40 bg-slate-600 text-white flex justify-start px-2" />
				</tr>
			</thead>
			<tbody class="contents">
				{#each months as monthKey}
					<div
						class="col-start-1 col-end-8 sticky top-7 h-7 z-40 bg-orange-300 flex justify-center items-center uppercase font-bold"
					>
						{getMonthNameFromKey(monthKey)}
					</div>
					{#each tableData[monthKey] as event (event.id)}
						{@const weekNr = getISOWeek(event.date)}
						<tr class="contents">
							<td class:odd-week={weekNr % 2} class="!pl-2">
								{getISOWeek(event.date)}
							</td>
							<td class:odd-week={weekNr % 2}>
								<DateTimePicker
									bind:date={event.date}
									bind:time={event.time}
									on:change={() => save(event)}
								/>
							</td>
							<td class:odd-week={weekNr % 2}
								>{event.date.toLocaleDateString(locale, {
									weekday: 'long'
								})}</td
							>
							<td class:odd-week={weekNr % 2}>
								<PersonSelect
									bind:name={event.assigneeName}
									bind:personId={event.assigneeId}
									{persons}
									on:change={() => save(event)}
								/>
							</td>
							<td class:odd-week={weekNr % 2} class=""
								><input
									class="block w-full h-full px-2 bg-slate-100 border border-slate-200 rounded-sm"
									bind:value={event.place}
									on:blur={() => save(event)}
								/></td
							>
							<td class:odd-week={weekNr % 2} class="p-0"
								><input
									class="block w-full h-full px-2 bg-slate-100 border border-slate-200 rounded-sm"
									bind:value={event.extraInfo}
									on:blur={() => save(event)}
								/></td
							>
							<td class:odd-week={weekNr % 2}>
								<Button
									outline
									size="xs"
									pill
									class="!p-2 border-transparent"
									on:click={() => deleteEvent(event)}
								>
									<Icon class="h-4 w-4 shrink-0" src={Trash} />
								</Button>
							</td>
						</tr>
					{/each}
				{/each}
			</tbody>
		</table>
	</div>
</MainLayout>

<style lang="postcss">
	td {
		@apply px-1 py-1 flex items-center;
	}

	.odd-week {
		@apply bg-orange-200;
	}
</style>
