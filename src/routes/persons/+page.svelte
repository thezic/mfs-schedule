<script lang="ts">
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import { Button } from 'flowbite-svelte';
	import { type Person, getPersons as getPersonsFromServer } from 'bindings';

	let persons: Person[] = [];

	async function getPersons() {
		persons = await getPersonsFromServer();
		console.log(persons);
	}

	getPersons();
</script>

<MainLayout>
	<div slot="header-right" class="flex-none">
		<Button href="/persons/new">Add user</Button>
	</div>
	<div class="px-8">
		<div class="flex items-center">
			<div class="flex-auto">
				<h1 class="text-base font-semibold leading-6 text-gray-900">Users</h1>
				<p class="mt-2 text-sm text-gray-700">A list of all the persons available for scheduling</p>
			</div>
			<div class="ml-16 mt-0 flex-none" />
		</div>
		<div class="mt-8 flow-root">
			<div class="-my-2 overflow-x-auto -mx-8">
				<div class="inline-block min-w-full py-2 align-middle px-8">
					<table class="min-w-full divide-y divide-gray-300">
						<thead>
							<tr>
								<th
									scope="col"
									class="py-3.5 pr-3 text-left text-sm font-semibold text-gray-900 pl-3">Name</th
								>
								<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
									>Comment</th
								>
							</tr>
						</thead>
						<tbody class="bg-white">
							{#each persons as person (person.id)}
								<tr class="even:bg-gray-50">
									<td class="whitespace-nowrap py-4 pr-3 text-sm font-medium text-gray-900 pl-3"
										>{person.name}</td
									>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{person.comment}</td
									>
									<td
										class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-3"
									>
										<a href={`/persons/${person.id}`} class="text-indigo-600 hover:text-indigo-900"
											>Edit</a
										>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</MainLayout>
