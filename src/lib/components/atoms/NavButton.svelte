<script lang="ts">
	import { page } from '$app/stores';
	import { Icon, type IconSource } from '@steeze-ui/svelte-icon';
	import { cx } from '$lib/utils/classes';

	export let href: string;
	export let name: string;
	export let absolute = false;
	export let icon: IconSource;

	$: active = absolute ? $page.url.pathname === href : $page.url.pathname.startsWith(href);
</script>

<a
	{href}
	class={cx(
		'text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold',
		active && 'text-white bg-gray-800'
	)}
>
	<Icon src={icon} class="h-6 w-6 shrink-0" />
	<span class="sr-only">{name}</span>
</a>
