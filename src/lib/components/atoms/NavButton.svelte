<script lang="ts">
	import { cx } from '$lib/utils/classes';
	export let href: string;
	export let name: string;
	export let active: boolean;
</script>

<a
	{href}
	class={cx(
		'text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold',
		active && 'text-white bg-gray-800'
	)}
>
	<slot />
	<span class="sr-only">{name}</span>
</a>
