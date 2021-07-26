#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::{command, State};

mod redis;
mod menu;

type Client = redis::Connection;

#[derive(Default)]
struct Context(
	Arc<Mutex<HashMap<String, Client>>>
);

/**
 * TODO: password, ssl, etc
 */
#[command]
fn redis_connect(label: String, host: String, port: u16, password: Option<String>, username: Option<String>, clients: State<Context>) {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let client = redis::connect(host, port, false, password, username);
	locker.insert(label, client);
	drop(locker);
}

#[command]
fn redis_select(label: String, namespaceid: String, clients: State<Context>) {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	redis::select(&mut client, namespaceid);
	drop(client); drop(locker);
}

#[command]
fn redis_disconnect(label: String, clients: State<Context>) {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	locker.remove(&label);
	redis::disconnect();
	drop(locker);
}

#[command]
fn redis_keylist(label: String, clients: State<Context>) -> Vec<String> {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	let keys = redis::keys(&mut client);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_lastsync(label: String, clients: State<Context>) -> String {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	let value = redis::lastsync(&mut client);
	drop(client); drop(locker);
	return value;
}

#[command]
fn redis_sync(label: String, timestamp: String, clients: State<Context>) {
	println!("inside sync! {}", timestamp);
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	redis::timestamp(&mut client, &timestamp);
	drop(client); drop(locker);
}

#[command]
fn redis_set(label: String, name: String, syncd: String, expires: Option<String>, metadata: Option<String>, clients: State<Context>) {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	redis::set(&mut client, name, syncd, expires, metadata);
	drop(client); drop(locker);
}

#[command]
fn redis_filter(label: String, pattern: String, clients: State<Context>) -> Vec<String> {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	let keys = redis::filter(&mut client, pattern);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_sort(label: String, descending: bool, clients: State<Context>) -> Vec<String> {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	let keys = redis::sort(&mut client, descending);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_details(label: String, key: String, clients: State<Context>) -> HashMap<String, String> {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	let keys = redis::details(&mut client, key);
	drop(client); drop(locker);
	return keys;
}

#[command]
fn redis_value(label: String, key: String, value: String, timestamp: String, mimetype: Option<String>, clients: State<Context>) {
	let mut locker = clients.0.lock().expect("could not lock mutex");
	let mut client = locker.get_mut(&label).expect("missing redis client");
	redis::value(&mut client, key, value, timestamp, mimetype);
	drop(client); drop(locker);
}

fn main() {
	tauri::Builder::default()
		.manage(
			Context::default()
		)
		.invoke_handler(tauri::generate_handler![
			redis_connect,
			redis_disconnect,
			redis_select,

			redis_keylist,
			redis_lastsync,

			redis_set,
			redis_sync,
			redis_filter,
			redis_sort,

			redis_details,
			redis_value,
		])
		.menu(
			menu::mainmenu()
		)
		.on_menu_event(|event| {
			println!("{:?}", event.menu_item_id());
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
