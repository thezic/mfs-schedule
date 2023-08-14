<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Input, Dropdown, DropdownItem, Helper } from 'flowbite-svelte';
	import type { Person } from '$lib/models/Person';
	export let name: string;
	export let personId: number | undefined;
	export let persons: Person[];

	let open = false;
	const dispatch = createEventDispatcher();

	function selectPerson(person: Person) {
		name = person.name;
		personId = person.id;
		open = false;
		dispatch('input', { name, personId });
		dispatch('change', { name, personId });
		dispatch('blur', { name, personId });
	}

	function onInput() {
		personId = undefined;
		dispatch('input', { name, personId });
	}

	console.log(name, personId, persons);
</script>

<Input
	bind:value={name}
	size="sm"
	on:blur={() => dispatch('blur', { name, personId })}
	on:change={() => dispatch('change', { name, personId })}
	on:input={onInput}
>
	<span slot="right" class="text-red-800 font-bold"
		>{#if name && !personId}!{/if}</span
	>
</Input>
<Dropdown bind:open>
	{#each persons as person}
		<DropdownItem on:click={() => selectPerson(person)}
			><div>{person.name}</div>
			<Helper>{person.comment}</Helper>
		</DropdownItem>
	{/each}
</Dropdown>
