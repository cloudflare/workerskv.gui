export function read(key: string) {
	return JSON.parse(localStorage.getItem(key) || '0');
}

export function write<T>(key: string, val: T) {
	localStorage.setItem(key, JSON.stringify(val));
}

export function remove(key) {
	localStorage.removeItem(key);
}
