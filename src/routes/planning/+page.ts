import { getPlannedEvents, getPersons } from 'bindings';
import type { PageLoad } from './$types';

export const ssr = false;

export const load = (async () => {
	const [events, persons] = await Promise.all([getPlannedEvents(), getPersons()]);

	return {
		events,
		persons
	};
}) satisfies PageLoad;
