import { formatDate } from '$lib/utils/date';
import type { MinistryEvent as MinistryEventDto } from 'bindings';

export class Time {
	hour: number;
	minute: number;

	constructor(hour: number, minute: number) {
		this.hour = hour;
		this.minute = minute;
	}

	static fromString(timeString: string) {
		const matches = timeString.match(/(?<h>\d\d):(?<m>\d\d)/);
		if (!matches?.groups) throw new Error(`Unable to parse timestring: '{timeString}'`);

		const hour = parseInt(matches.groups['h']);
		const minute = parseInt(matches.groups['m']);

		return new Time(hour, minute);
	}
}

export class MinistryEvent {
	id: number;
	date: Date;
	time?: string;
	place: string;
	extraInfo: string;
	assigneeName: string;
	assigneeId?: number;

	constructor(fromData: MinistryEventDto) {
		this.id = fromData.id;
		this.date = new Date(Date.parse(fromData.date));
		this.time = fromData.time ?? undefined;
		this.place = fromData.place;
		this.extraInfo = fromData.extraInfo;
		(this.assigneeName = fromData.assigneeName),
			(this.assigneeId = fromData.assigneeId ?? undefined);
	}

	asDto(): MinistryEventDto {
		return {
			id: this.id,
			assigneeId: this.assigneeId ?? null,
			place: this.place,
			assigneeName: this.assigneeName,
			extraInfo: this.extraInfo,
			date: formatDate(this.date),
			time: this.time ?? null
		};
	}
}
