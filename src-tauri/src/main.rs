#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use csv_compare_js_lib::compare_csv_contents as lib_compare_csv_contents;
use csv_compare_js_lib::ComparisonResult;

#[tauri::command]
fn compare_csv_contents(file1_content: String, file2_content: String) -> Result<ComparisonResult, String> {
    lib_compare_csv_contents(file1_content, file2_content)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compare_csv_contents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}