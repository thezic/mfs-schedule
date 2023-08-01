<script lang="ts">
	import { page } from '$app/stores';
	import MainLayout from '$lib/components/layouts/MainLayout.svelte';
</script>

<MainLayout>
	<h1>Person with id {$page.params.id}</h1>
</MainLayout>
