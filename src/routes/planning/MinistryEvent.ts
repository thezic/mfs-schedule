import type { MinistryEvent as MinistryEventDto } from 'bindings';

export class MinistryEvent {
	id: number;
	scheduledTime: Date;
	place: string;
	extraInfo: string;
	assigneeName: string;
	assigneeId?: number;

	constructor(fromData: MinistryEventDto) {
		this.id = fromData.id;
		this.scheduledTime = new Date(Date.parse(fromData.scheduledTime));
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
			scheduledTime: this.scheduledTime.toISOString()
		};
	}
}
