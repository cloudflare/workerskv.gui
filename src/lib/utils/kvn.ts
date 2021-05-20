import { getClient } from '@tauri-apps/api/http';
import type { HttpOptions } from '@tauri-apps/api/http';

// sent to Redis
export interface Key {
	name: string;
	syncd: string;
	expires?: string;
	metadata?: string;
	lastupdate?: string;
}

// come via REST API
export interface KeyInfo {
	name: string;
	expiration?: number;
	metadata?: Dict<any>;
}

interface KeyList {
	success: boolean;
	messages: string[];
	errors: string[];
	result: KeyInfo[];
	result_info: {
		count: number;
		cursor: string;
	}
}

async function send<T>(url: string, options: Partial<HttpOptions> = {}) {
	const client = await getClient();
	const method = options.method || 'GET';

	options.headers = {
		...options.headers,
		Origin: 'https://github.com'
	};

	return client.request<T>({ ...options, url, method });
}

export async function * list(
	accountid: string,
	namespaceid: string,
	token: string,
): AsyncGenerator<{ done: boolean; keys: KeyInfo[] }> {
	let cursor = '', headers = { Authorization: `Bearer ${token}` };
	// @see https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys
	let endpoint = new URL(`https://api.cloudflare.com/client/v4/accounts/${accountid}/storage/kv/namespaces/${namespaceid}/keys?limit=1000`);

	while (true) {
		if (cursor) endpoint.searchParams.set('cursor', cursor);
		let { data } = await send<KeyList>(endpoint.href, { headers });

		if (data.success) {
			cursor = data.result_info.cursor;

			yield {
				done: cursor.length === 0,
				keys: data.result,
			};
		} else {
			break;
		}
	}
}
