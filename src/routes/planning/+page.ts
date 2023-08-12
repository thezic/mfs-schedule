import { getPlannedEvents, type MinistryEvent } from 'bindings';
import type { PageLoad } from './$types';

export const ssr = false;

export const load = (async () => {
	const loadedData = await getPlannedEvents();
	console.log('planned', loadedData);
	return {
		events: loadedData as MinistryEvent[]
	};
}) satisfies PageLoad;
