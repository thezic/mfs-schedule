import { browser } from '$app/environment';
import { getPersons } from 'bindings';

export const ssr = false;

export async function load() {
	const persons = await getPersons();

	return {
		persons
	};
}
