<script lang="ts">
	import NavButton from '$lib/components/atoms/NavButton.svelte';
	import { cx } from '$lib/utils/classes';
	import { Users, Calendar, QueueList } from '@steeze-ui/heroicons';
</script>

<div class="h-screen">
	<!-- Static sidebar -->
	<div class="fixed inset-y-0 left-0 z-50 block w-20 overflow-y-auto bg-gray-900 pb-4">
		<div class="flex h-16 shrink-0 items-center justify-center">
			<img
				class="h-8 w-auto"
				src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
				alt="Your Company"
			/>
		</div>
		<nav class="mt-8">
			<ul role="list" class="flex flex-col items-center space-y-1">
				<li>
					<NavButton href="/planning" name="Planning" icon={QueueList} />
				</li>
				<li>
					<NavButton href="/persons" name="Persons" icon={Users} />
				</li>
				<li>
					<NavButton href="/schedule" name="schedule" icon={Calendar} />
				</li>
			</ul>
		</nav>
	</div>
	<div />

	<div class="pl-20 flex flex-col h-full overflow-hidden">
		<div
			class="flex h-16 shrink-0 grow-0 items-center gap-x-4 border-b border-gray-200 bg-white shadow-sm px-8 justify-between"
		>
			<slot name="header">
				<div><slot name="header-left" /></div>
				<div><slot name="header-right" /></div>
			</slot>
		</div>

		<div class="flex flex-row flex-grow h-full overflow-hidden">
			<main class="flex-grow overflow-y-hidden">
				<div class={cx($$slots.aside && 'xl:pl-96', 'h-full')}>
					<div class="px-8 py-6 h-full">
						<!-- Main area -->
						<slot />
					</div>
				</div>
			</main>

			{#if $$slots.aside}
				<aside
					class="fixed inset-y-0 left-20 top-16 hidden w-96 overflow-y-auto border-r border-gray-200 px-4 py-6 sm:px-6 lg:px-8 xl:block"
				>
					<!-- Secondary column (hidden on smaller screens) -->
					<slot name="aside" />
				</aside>
			{/if}
		</div>
	</div>
</div>
