#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(move |app| {
      let window = app.get_window("overlay").unwrap();
      tauri::async_runtime::spawn(async move {
        //TODO: The goal here is to set the size of windows based on the current monitor
        // let monitor = window.primary_monitor().unwrap();
        // match monitor {
        //   Some(m) => {
        //     let size = *m.size();
        //     window.set_size(size);
        //   }
        //   None => println!("Error getting the primary monitor"),
        // }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
