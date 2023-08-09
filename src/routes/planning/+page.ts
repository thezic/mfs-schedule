import type { MinistryEvent } from 'bindings';
import type { PageLoad } from './$types';

export const ssr = false;

export const load = (() => {
	const loadedData = [
		{
			id: 1,
			place: 'Some place',
			assigneeName: 'Simon Dahlberg',
			assigneeId: 1,
			extraInfo: 'Extra info',
			scheduledTime: '2023-08-05T15:00Z'
		}
	] as MinistryEvent[];

	return {
		events: loadedData.map((event) => ({
			id: event.id,
			scheduledTime: new Date(Date.parse(event.scheduledTime)),
			place: event.place,
			extraInfo: event.extraInfo,
			assigneeName: event.assigneeName,
			assigneeId: event.assigneeId
		}))
	};
}) satisfies PageLoad;
