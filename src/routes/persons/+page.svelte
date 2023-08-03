<script lang="ts">
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import { invoke } from '@tauri-apps/api';

	let persons = [
		{ id: 1, name: 'Simon Dahlberg' },
		{ id: 2, name: 'Karl Johansson' }
	];

	async function getPersons() {
		const names = (await invoke('get_persons')) as string[];
		persons = names.map((name: string, id: number) => ({
			name,
			id
		}));
	}

	getPersons();
</script>

<MainLayout>
	<div class="px-8">
		<div class="flex items-center">
			<div class="flex-auto">
				<h1 class="text-base font-semibold leading-6 text-gray-900">Users</h1>
				<p class="mt-2 text-sm text-gray-700">A list of all the persons available for scheduling</p>
			</div>
			<div class="ml-16 mt-0 flex-none">
				<a
					href="/persons/new"
					class="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>
					Add user</a
				>
			</div>
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
									>Title</th
								>
								<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
									>Email</th
								>
								<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
									>Role</th
								>
								<th scope="col" class="relative py-3.5 pl-3 pr-3">
									<span class="sr-only">Edit</span>
								</th>
							</tr>
						</thead>
						<tbody class="bg-white">
							{#each persons as person (person.id)}
								<tr class="even:bg-gray-50">
									<td class="whitespace-nowrap py-4 pr-3 text-sm font-medium text-gray-900 pl-3"
										>{person.name}</td
									>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500"
										>Front-end Developer</td
									>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500"
										>lindsay.walton@example.com</td
									>
									<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">Member</td>
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
