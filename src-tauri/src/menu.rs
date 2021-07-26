use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn mainmenu() -> Menu {
	// NOTE: custom keybinds not yet supported
	let new_window = CustomMenuItem::new("new:window", "New Window");
	let new_issue = CustomMenuItem::new("new:issue", "Report an Issue");

	let menu_app = Menu::new()
		.add_native_item(MenuItem::About(
			"Workers KV".to_string()
		))
		.add_native_item(MenuItem::Services)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::Hide)
		.add_native_item(MenuItem::HideOthers)
		.add_native_item(MenuItem::ShowAll)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::Quit);

	let menu_file = Menu::new()
		.add_item(new_window)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::CloseWindow);

	let menu_edit = Menu::new()
		.add_native_item(MenuItem::Undo)
		.add_native_item(MenuItem::Redo)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::Cut)
		.add_native_item(MenuItem::Copy)
		.add_native_item(MenuItem::Paste)
		.add_native_item(MenuItem::SelectAll);

	let menu_view = Menu::new()
		.add_native_item(MenuItem::EnterFullScreen);

	let menu_window = Menu::new()
		.add_native_item(MenuItem::Minimize)
		.add_native_item(MenuItem::Zoom);

	let menu_help = Menu::new()
		.add_item(new_issue);

	Menu::new()
		.add_submenu(Submenu::new("Workers KV", menu_app))
		.add_submenu(Submenu::new("File", menu_file))
		.add_submenu(Submenu::new("Edit", menu_edit))
		.add_submenu(Submenu::new("View", menu_view))
		.add_submenu(Submenu::new("Window", menu_window))
		.add_submenu(Submenu::new("Help", menu_help))
}
