/// <reference types="@sveltejs/kit" />

type Dict<T> = Record<string, T>;

declare const __TAURI__: {
	invoke: typeof import('@tauri-apps/api/tauri').invoke;
}
