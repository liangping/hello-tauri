#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn my_custom_command(msg: String) -> String {
  println!("I was invoked from JS! {}", msg);
  return "Hello from Rust".into()
}

fn main() {
  tauri::Builder::default()
      // This is where you pass in your commands
      .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
