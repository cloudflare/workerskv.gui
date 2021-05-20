/**
 * current time, in seconds
 */
export function timestamp(): string {
	return String(Date.now() / 1e3 | 0);
}

/** @example "May 20, 2021 at 2:09 PM" */
export function date(seconds: string): string {
	let date = new Date(+seconds * 1e3);
	let output = date.toLocaleDateString(undefined, {
		year: 'numeric',
		day: 'numeric',
		month: 'short',
	});

	output += ' @ ';

	output += date.toLocaleTimeString(undefined, {
		minute: '2-digit',
		hour: '2-digit',
	});

	return output;
}
