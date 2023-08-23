export function cx(...classes: (number | string | boolean | undefined)[]) {
	return classes.filter((cls): cls is string => !!cls).join(' ');
}
