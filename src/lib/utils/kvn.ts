import { getClient } from '@tauri-apps/api/http';
import type { HttpOptions } from '@tauri-apps/api/http';

async function send<T>(url: string, options: Partial<HttpOptions> = {}) {
	const client = await getClient();
	const method = options.method || 'GET';

	options.headers = {
		...options.headers,
		Origin: 'https://github.com'
	};

	return client.request<T>({ ...options, url, method });
}

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

interface Result {
	success: boolean;
	messages: string[];
	errors: string[];
	result: KeyInfo[];
	result_info: {
		count: number;
		cursor: string;
	}
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
		let { data } = await send<Result>(endpoint.href, { headers });

		if (data.success) {
			cursor = data.result_info.cursor;

			yield {
				done: !!cursor,
				keys: data.result,
			};
		} else {
			break;
		}
	}
}
