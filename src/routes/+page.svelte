<script lang="ts">
	import NavButton from '$lib/components/atoms/NavButton.svelte';
	import { Home, Users, Calendar } from '@steeze-ui/heroicons';
</script>

<!--
  This example requires updating your template:

  ```
  <html class="h-full bg-white">
  <body class="h-full">
  ```
-->
<div>
	<!-- Static sidebar for desktop -->
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
					<!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
					<NavButton href="#" name="Dashboard" icon={Home} active />
				</li>
				<li>
					<NavButton href="#" name="Persons" icon={Users} />
				</li>
				<li>
					<NavButton href="#" name="schedule" icon={Calendar} />
				</li>
			</ul>
		</nav>
	</div>

	<main class="lg:pl-20">
		<div class="xl:pl-96">
			<div class="px-4 py-10 sm:px-6 lg:px-8 lg:py-6">
				<!-- Main area -->
			</div>
		</div>
	</main>

	<aside
		class="fixed inset-y-0 left-20 hidden w-96 overflow-y-auto border-r border-gray-200 px-4 py-6 sm:px-6 lg:px-8 xl:block"
	>
		<!-- Secondary column (hidden on smaller screens) -->
	</aside>
</div>
