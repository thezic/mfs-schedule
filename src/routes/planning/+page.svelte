<script lang="ts">
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import type { PageData } from './$types';
	// import { type MinistryEvent } from 'bindings';
	export let data: PageData;

	console.log(data.events);
</script>

<MainLayout>
	<div>
		<table class="min-w-full divide-y divide-gray-300">
			<tbody>
				{#each data.events as event (event.id)}
					<tr>
						<td>{event.scheduledTime}</td>
						<td>{event.assigneeName}</td>
						<td>{event.place}</td>
						<td>{event.extraInfo}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</MainLayout>
