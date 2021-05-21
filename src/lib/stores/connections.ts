import { writable } from 'svelte/store';
import * as storage from '$lib/utils/storage';

export interface Connection {
	nickname: string;
	color: `#${string}`;
	/** redis host */
	host: string;
	/** redis port */
	port: number;
	/** redis username */
	username: Nullable<string>;
	/** redis password */
	password: Nullable<string>;
	/** kv namespaceid */
	namespaceid: string;
	/** cflare accountid */
	accountid: string;
	/** cflare accesstoken */
	accesstoken: string;
}

export const active = writable<Partial<Connection>>({
	// empty
});

export function refresh(): Connection[] {
	return storage.read('favorites') || [];
}

export function update(list: Connection[]) {
	storage.write('favorites', list);
}

export function select(conn: Connection) {
	active.set(conn);
}
