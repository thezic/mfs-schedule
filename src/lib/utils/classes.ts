export function cx(...classes: (string | boolean | undefined)[]) {
	return classes.filter((cls): cls is string => !!cls).join(' ');
}
