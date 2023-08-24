import { formatDate } from '$lib/utils/date';
import type { MinistryEvent as MinistryEventDto, NewMinistryEvent } from 'bindings';
import { addDays, startOfWeek } from 'date-fns';

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

export interface UndatedEvent {
	day: number;
	time?: string;
	place: string;
	extraInfo: string;
}

export interface TemplatedEvent extends Omit<UndatedEvent, 'day'> {
	date: Date;
}

export class MinistryEventTemplate {
	template: UndatedEvent[] = [];
	name: string;
	id: number;

	constructor(id: number, name: string, template: UndatedEvent[]) {
		this.id = id;
		this.name = name;
		this.template = template;
	}

	*atWeek(week: Date) {
		const monday = startOfWeek(week);
		for (const template of this.template) {
			yield {
				date: formatDate(addDays(monday, template.day)),
				extraInfo: template.extraInfo,
				place: template.place,
				time: template.time ?? null,
				assigneeId: null,
				assigneeName: ''
			} satisfies NewMinistryEvent;
		}
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
