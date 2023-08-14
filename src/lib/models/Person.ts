import type { Person as PersonDto } from 'bindings';

export class Person {
	id: number;
	name = '';
	comment = '';

	constructor(fromDto: PersonDto) {
		this.id = fromDto.id;
		this.name = fromDto.name;
		this.comment = fromDto.comment;
	}
}
