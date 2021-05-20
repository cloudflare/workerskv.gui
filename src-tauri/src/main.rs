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
}

#[command]
fn redis_disconnect(state: State<Context>) {
	let mut locker = state.client.lock().expect("could not lock mutex");
	redis::disconnect();
	*locker = None;
}

#[command]
fn redis_load(state: State<Context>) -> Vec<String> {
	let mut locker = state.client.lock().expect("could not lock mutex");
	let mut client = locker.as_mut().expect("missing redis client");
	redis::keys(&mut client)
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
			redis_load,

			// stateful_command,
			window_label,
			print_dirs,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
