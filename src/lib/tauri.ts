export function dispatch<T>(command: string, args?: Dict<string|number>): Promise<T> {
	return __TAURI__.invoke(command, args);
}
