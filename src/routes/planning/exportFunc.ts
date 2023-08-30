import { formatDate } from '$lib/utils/date';
import z from 'zod';

const KEY = 'EXPORT_DATA';

const DATE_REGEX = /\d{4}-\d{2}-\d{2}/;

const schema = z.object({
	dateFrom: z.string().regex(DATE_REGEX),
	dateUntil: z.string().regex(DATE_REGEX),
	text: z.string().optional()
});

type Input = z.input<typeof schema>;
type ExportData = z.output<typeof schema>;

export function load() {
	const data = localStorage.getItem(KEY);
	if (!data) return undefined;

	const result = schema.safeParse(data);

	if (!result.success) {
		console.error('unable to parse data', data);
		localStorage.removeItem(KEY);
		return undefined;
	}

	return result.data;
}

export function save(data: ExportData) {
	localStorage.setItem(
		KEY,
		JSON.stringify({
			...data
		} satisfies Input)
	);
}
