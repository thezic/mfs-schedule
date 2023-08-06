<script lang="ts">
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
	import { invoke } from '@tauri-apps/api';

	let name = '';

	async function save() {
		await invoke('create_person', {
			newPerson: {
				name
			}
		});
	}
</script>

<MainLayout>
	<form on:submit={save}>
		<div class="space-y-12 sm:space-y-16">
			<div>
				<h2 class="text-base font-semibold leading-7 text-gray-900">Personal Information</h2>
				<p class="mt-1 max-w-2xl text-sm leading-6 text-gray-600">
					Some information, don't know what it should be yet
				</p>

				<div
					class="mt-10 space-y-8 border-b border-gray-900/10 pb-12 sm:space-y-0 sm:divide-y sm:divide-gray-900/10 sm:border-t sm:pb-0"
				>
					<div class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:py-6">
						<label for="name" class="block text-sm font-medium leading-6 text-gray-900 sm:pt-1.5"
							>Name</label
						>
						<div class="mt-2 sm:col-span-2 sm:mt-0">
							<input
								type="text"
								name="name"
								id="name"
								class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm sm:leading-6"
								bind:value={name}
							/>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="mt-6 flex items-center justify-end gap-x-6">
			<button type="button" class="text-sm font-semibold leading-6 text-gray-900">Cancel</button>
			<button
				type="submit"
				class="inline-flex justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>Save</button
			>
		</div>
	</form>
</MainLayout>
