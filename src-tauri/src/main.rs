#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// extern crate redis;
use tauri::{command, State, Window};
mod redis;

#[derive(Debug)]
struct Context {
  value: u64,
	label: String,
}

#[command]
fn stateful_command(argument: Option<String>, state: State<Context>) {
  println!("{:?} {:?}", argument, state.inner());
}

// ---- Redis Commands ----

/**
 * TODO: password, ssl, etc
 */
#[command]
fn redis_connect(host: String, port: u16) {
	let mut conn = redis::connect(host, port, false);
	println!("i am here");
	let _: () = redis::run(&mut conn, "SET", &["foo", "bar"]).expect("failed to execute SET for 'foo'");
	println!("i ran SET");
}

// ------------------------ Commands using Window ------------------------
#[command]
fn window_label(window: Window) {
  println!("window label: {}", window.label());
}

fn main() {
	tauri::Builder::default()
		.manage(Context {
			value: 0,
			label: "Hello".into(),
		})
		.invoke_handler(tauri::generate_handler![
			redis_connect,
			stateful_command,
			window_label,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
