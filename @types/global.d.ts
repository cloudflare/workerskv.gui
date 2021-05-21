/// <reference types="@sveltejs/kit" />

type Dict<T> = Record<string, T>;
type Nullable<T> = T | null;

declare const __TAURI__: {
	invoke: typeof import('@tauri-apps/api/tauri').invoke;
	__currentWindow: import('@tauri-apps/api/window').WebviewWindowHandle;
}
