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

	// import { type MinistryEvent } from 'bindings';
	export let data: PageData;

	console.log(data.events);
</script>

<MainLayout>
	<div slot="header-right" class="flex justify-between gap-4">
		<Button>Add from schedule</Button>
		<Button>Add meeting</Button>
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
				{#each data.events as event (event.id)}
					<TableBodyRow>
						<TableBodyCell>
							<DateTimePicker bind:value={event.scheduledTime} />
						</TableBodyCell>
						<TableBodyCell
							>{event.scheduledTime.toLocaleDateString(navigator.language, {
								weekday: 'long'
							})}</TableBodyCell
						>
						<TableBodyCell><Input bind:value={event.assigneeName} size="sm" /></TableBodyCell>
						<TableBodyCell><Input bind:value={event.place} size="sm" /></TableBodyCell>
						<TableBodyCell><Input bind:value={event.extraInfo} size="sm" /></TableBodyCell>
						<TableBodyCell class="flex justify-end pr-0"
							><Button outline size="xs" pill class="!p-2 border-transparent"
								><Icon class="h-4 w-4 shrink-0" src={Trash} /></Button
							></TableBodyCell
						>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</MainLayout>
