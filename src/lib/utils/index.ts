/**
 * current time, in seconds
 */
export function timestamp() {
	return String(Date.now() / 1e3 | 0);
}
