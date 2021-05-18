export function dispatch(command: string, args: Dict<string|number>) {
	return __TAURI__.invoke(command, args);
}
