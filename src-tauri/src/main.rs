#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::{command, State, Window};

mod redis;

type Client = Option<redis::Connection>;

struct Context {
  client: Arc<Mutex<Client>>,
}

// ---- Redis Commands ----

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
	drop(locker);
	return keys;
}

#[command]
fn redis_sync(timestamp: String, state: State<Context>) {
	println!("inside sync! {}", timestamp);
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	redis::timestamp(&mut client, &timestamp);
	drop(locker);
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
fn print_dirs() {
	println!("config_dir: {:?}", tauri::api::path::config_dir());
	println!("cache_dir: {:?}", tauri::api::path::cache_dir());
	println!("data_dir: {:?}", tauri::api::path::data_dir());
	println!("public_dir: {:?}", tauri::api::path::public_dir());
}

// ------------------------ Commands using Window ------------------------
#[command]
fn window_label(window: Window) {
  println!("window label: {}", window.label());
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

			// stateful_command,
			window_label,
			print_dirs,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
