export function formatDate(date: Date) {
	// Take Timezone offset into account, as toISOString returns a string in UTC time
	return new Date(date.getTime() - date.getTimezoneOffset() * 60000).toISOString().split('T')[0];
}
