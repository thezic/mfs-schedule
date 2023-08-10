<script lang="ts">
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Button,
		Input
	} from 'flowbite-svelte';
	import type { PageData } from './$types';
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import { Trash } from '@steeze-ui/heroicons';
	import { Icon } from '@steeze-ui/svelte-icon';
	import DateTimePicker from './DateTimePicker.svelte';
	import {
		createEvent as createMinistryEvent,
		deleteEvent as deleteMinistryEvent,
		updateEvent
	} from 'bindings';
	import { MinistryEvent } from './MinistryEvent';

	// import { type MinistryEvent } from 'bindings';
	export let data: PageData;
	let events = data.events.map((event) => new MinistryEvent(event));

	async function createEvent() {
		const lastEvent = events.length ? events[data.events.length - 1] : undefined;
		const scheduledTime =
			lastEvent?.scheduledTime.toISOString() ?? new Date(Date.now()).toISOString();

		const newEvent = await createMinistryEvent({
			place: '',
			scheduledTime,
			extraInfo: '',
			assigneeName: '',
			assigneeId: null
		});

		console.log(newEvent);
		events.push(new MinistryEvent(newEvent));
		events = events;
	}

	async function deleteEvent(eventToDelete: MinistryEvent) {
		console.log('delete event ', eventToDelete);
		await deleteMinistryEvent(eventToDelete.id);

		events = events.filter((event) => event.id !== eventToDelete.id);
		console.log('event was deleted', eventToDelete);
	}

	async function save(event: MinistryEvent) {
		const updatedData = await updateEvent(event.asDto());

		events.splice(events.indexOf(event), 1, new MinistryEvent(updatedData));
		events = events;
	}
</script>

<MainLayout>
	<div slot="header-right" class="flex justify-between gap-4">
		<Button>Add from schedule</Button>
		<Button on:click={createEvent}>Add meeting</Button>
	</div>
	<div>
		<Table>
			<TableHead>
				<TableHeadCell>Time</TableHeadCell>
				<TableHeadCell>Day</TableHeadCell>
				<TableHeadCell>Assignee</TableHeadCell>
				<TableHeadCell>Place</TableHeadCell>
				<TableHeadCell>Extra info</TableHeadCell>
				<TableHeadCell />
			</TableHead>
			<TableBody>
				{#each events as event (event.id)}
					<TableBodyRow>
						<TableBodyCell>
							<DateTimePicker bind:value={event.scheduledTime} on:change={() => save(event)} />
						</TableBodyCell>
						<TableBodyCell
							>{event.scheduledTime.toLocaleDateString(navigator.language, {
								weekday: 'long'
							})}</TableBodyCell
						>
						<TableBodyCell
							><Input
								bind:value={event.assigneeName}
								size="sm"
								on:blur={() => save(event)}
							/></TableBodyCell
						>
						<TableBodyCell
							><Input
								bind:value={event.place}
								size="sm"
								on:blur={() => save(event)}
							/></TableBodyCell
						>
						<TableBodyCell
							><Input
								bind:value={event.extraInfo}
								size="sm"
								on:blur={() => save(event)}
							/></TableBodyCell
						>
						<TableBodyCell class="flex justify-end pr-0">
							<Button
								outline
								size="xs"
								pill
								class="!p-2 border-transparent"
								on:click={() => deleteEvent(event)}
							>
								<Icon class="h-4 w-4 shrink-0" src={Trash} />
							</Button>
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</MainLayout>
