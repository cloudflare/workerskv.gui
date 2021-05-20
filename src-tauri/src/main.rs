#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::{command, State};

mod redis;

type Client = Option<redis::Connection>;

struct Context {
  client: Arc<Mutex<Client>>,
}

/**
 * TODO: password, ssl, etc
 */
#[command]
fn redis_connect(host: String, port: u16, state: State<Context>) {
	let mut locker = state.client.lock().expect("could not lock mutex");
	*locker = Some(redis::connect(host, port, false));
	drop(locker);
}

#[command]
fn redis_disconnect(state: State<Context>) {
	let mut locker = state.client.lock().expect("could not lock mutex");
	redis::disconnect();
	*locker = None;
	drop(locker);
}

#[command]
fn redis_keylist(state: State<Context>) -> Vec<String> {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	let keys = redis::keys(&mut client);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_sync(timestamp: String, state: State<Context>) {
	println!("inside sync! {}", timestamp);
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	redis::timestamp(&mut client, &timestamp);
	drop(client); drop(locker);
}

#[command]
fn redis_set(name: String, syncd: String, expires: Option<String>, metadata: Option<String>, state: State<Context>) {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	redis::set(&mut client, name, syncd, expires, metadata);
	drop(client); drop(locker);
}

#[command]
fn redis_filter(pattern: String, state: State<Context>) -> Vec<String> {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	let keys = redis::filter(&mut client, pattern);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_sort(descending: bool, state: State<Context>) -> Vec<String> {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	let keys = redis::sort(&mut client, descending);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_details(key: String, state: State<Context>) -> HashMap<String, String> {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	let keys = redis::details(&mut client, key);
	drop(client); drop(locker);
	return keys;
}

fn main() {
	tauri::Builder::default()
		.manage(Context {
			client: Arc::new(Mutex::new(None)),
		})
		.invoke_handler(tauri::generate_handler![
			redis_connect,
			redis_disconnect,

			redis_set,
			redis_sync,
			redis_keylist,
			redis_filter,
			redis_sort,

			redis_details,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
