<script lang="ts">
	import NavButton from '$lib/components/atoms/NavButton.svelte';
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
					<NavButton href="#" name="Dashboard" active>
						<svg
							class="h-6 w-6 shrink-0"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
							/>
						</svg>
					</NavButton>
				</li>
				<li>
					<NavButton href="#" name="Persons">
						<svg
							class="h-6 w-6 shrink-0"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
							/>
						</svg>
					</NavButton>
				</li>
				<li>
					<NavButton href="#" name="schedule">
						<svg
							class="h-6 w-6 shrink-0"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
							/>
						</svg></NavButton
					>
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
