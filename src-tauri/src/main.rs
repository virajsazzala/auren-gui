#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;

#[command]
fn search(query: String) -> Vec<String> {

	// dummy search
	let mut result = Vec::new();

	let n = query.parse::<usize>().unwrap_or(0);
	for _ in 0..n {
		result.push(format!("{} {}", "file", query));
	}

	result
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![search])
		.run(tauri::generate_context!())
		.expect("error while running tauri app");
}