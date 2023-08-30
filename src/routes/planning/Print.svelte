<script lang="ts">
	import { Button, CloseButton, Drawer, Input, Label, Textarea } from 'flowbite-svelte';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { Printer } from '@steeze-ui/heroicons';
	import { formatDate } from '$lib/utils/date';
	import { sineIn } from 'svelte/easing';
	import { exportPdf } from 'bindings';
	import { load, save } from './exportFunc';

	let hidden = true;

	const data = load();
	let dateFrom = data?.dateFrom;
	let dateUntil = data?.dateUntil;
	let text = data?.text ?? '';

	const transitionParams = {
		x: 320,
		duration: 200,
		easing: sineIn
	};

	function assert(condition: unknown, message?: string): asserts condition {
		if (!condition) throw new Error(message);
	}

	async function exportProgram(e: SubmitEvent) {
		e.preventDefault();
		assert(dateFrom && dateUntil, 'dates are undefined');

		const from = formatDate(new Date(Date.parse(dateFrom)));
		const until = formatDate(new Date(Date.parse(dateUntil)));

		console.log(await exportPdf(from, until, { text }));

		save({
			dateFrom: from,
			dateUntil: until,
			text
		});
	}
</script>

<Button on:click={() => (hidden = false)}><Icon class="w-4 h-4" src={Printer} /></Button>
<Drawer bind:hidden placement="right" transitionType="fly" {transitionParams}>
	<div class="flex items-center">
		<h5
			class="inline-flex items-center mb-6 text-base font-semibold text-gray-500 uppercase dark:text-gray-400"
		>
			EXPORT PROGRAM
		</h5>
		<CloseButton on:click={() => (hidden = true)} class="mb-4 dark:text-white" />
	</div>
	<form on:submit={(e) => exportProgram(e)}>
		<div class="mb-6">
			<Label for="name" class="block mb-2">From</Label>
			<Input id="name" name="from" type="date" required placeholder="From" bind:value={dateFrom} />
		</div>
		<div class="mb-6">
			<Label for="bland" class="block mb-2">To</Label>
			<Input id="bland" name="to" type="date" required placeholder="To" bind:value={dateUntil} />
		</div>
		<div class="mb-6">
			<Label for="brand" class="mb-2">Extra text</Label>
			<Textarea placeholder="Enter event description here" rows="4" name="text" bind:value={text} />
		</div>
		<div class="bottom-0 left-0 flex justify-center w-full pb-4 space-x-4 md:px-4 md:absolute">
			<Button type="submit" class="w-full">Export</Button>
			<Button class="w-full" color="light" on:click={() => (hidden = true)}>
				<svg
					aria-hidden="true"
					class="w-5 h-5 -ml-1 sm:mr-1"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/></svg
				>
				Cancel
			</Button>
		</div>
	</form>
</Drawer>
