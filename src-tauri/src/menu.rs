use tauri::{CustomMenuItem, Menu, MenuItem};

pub fn mainmenu() -> Vec<Menu<String>> {
	// NOTE: custom keybinds not yet supported
	let new_window = MenuItem::Custom(
		CustomMenuItem::new("new:window".into(), "New Window")
	);

	let new_issue = MenuItem::Custom(
		CustomMenuItem::new("new:issue".into(), "Report an Issue")
	);

	#[cfg(any(target_os = "linux", target_os = "macos"))]
	let menu = {
		vec![
			Menu::new("Workers KV", vec![
				MenuItem::About(
					"Workers KV".to_string()
				),
				MenuItem::Services,
				MenuItem::Separator,
				MenuItem::Hide,
				MenuItem::HideOthers,
				MenuItem::ShowAll,
				MenuItem::Separator,
				MenuItem::Quit,
			]),
			Menu::new("File", vec![
				new_window,
				MenuItem::Separator,
				MenuItem::CloseWindow,
			]),
			Menu::new("Edit", vec![
				MenuItem::Undo,
				MenuItem::Redo,
				MenuItem::Separator,
				MenuItem::Cut,
				MenuItem::Copy,
				MenuItem::Paste,
				MenuItem::SelectAll,
			]),
			Menu::new("View", vec![
				MenuItem::EnterFullScreen
			]),
			Menu::new("Window", vec![
				MenuItem::Minimize,
				MenuItem::Zoom,
			]),
			Menu::new("Help", vec![
				new_issue
			]),
		]
	};

	// ATTENTION
	// Windows only supports custom menus, for the time being.
	// ---
	// Any `MenuItem::*` will not render;
	// @see https://github.com/cloudflare/workerskv.gui/issues/7
	// We could use custom menus via `Menu::new()` & EventLoop
	// ...or wait for the next `tauri.Menu` version w/ fixes.
	#[cfg(target_os = "windows")]
	let menu = {
		vec![
			// Menu::new("Workers KV", vec![
			// 	MenuItem::About(
			// 		"Workers KV".to_string()
			// 	),
			// 	MenuItem::Services,
			// 	MenuItem::Separator,
			// 	MenuItem::Hide,
			// 	MenuItem::HideOthers,
			// 	MenuItem::ShowAll,
			// 	MenuItem::Separator,
			// 	MenuItem::Quit,
			// ]),
			Menu::new("File", vec![
				new_window,
				// MenuItem::Separator,
				// MenuItem::CloseWindow,
			]),
			// Menu::new("Edit", vec![
			// 	MenuItem::Undo,
			// 	MenuItem::Redo,
			// 	MenuItem::Separator,
			// 	MenuItem::Cut,
			// 	MenuItem::Copy,
			// 	MenuItem::Paste,
			// 	MenuItem::SelectAll,
			// ]),
			// Menu::new("View", vec![
			// 	MenuItem::EnterFullScreen
			// ]),
			// Menu::new("Window", vec![
			// 	MenuItem::Minimize,
			// 	MenuItem::Zoom,
			// ]),
			Menu::new("Help", vec![
				new_issue
			]),
		]
	};

	menu
}
