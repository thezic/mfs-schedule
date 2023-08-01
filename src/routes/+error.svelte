<script lang="ts">
	import { page } from '$app/stores';
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
</script>

<MainLayout>
	<div>Something went wrong ({$page.status})</div>
	<div>path {$page.url}</div>
</MainLayout>
