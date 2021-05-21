export function dispatch<T>(command: string, args: Dict<string|number|boolean> = {}): Promise<T> {
	args.label = __TAURI__.__currentWindow.label;
	return __TAURI__.invoke(command, args);
}
