#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(move |app| {
      let obj_opts = app.get_window("objectives");
      match obj_opts {
        Some(obj_window) => {
          tauri::async_runtime::spawn(async move {
            //TODO: The goal here is to set the size of windows based on the current monitor
            let monitor = obj_window.primary_monitor();
            match monitor {
              Ok(monitor_opt) => match monitor_opt {
                Some(m) => {
                  // let size = *m.size();
                }
                None => {} // TODO: figure out what to do here
              },
              Err(e) => {} // TODO: figure out what I'm doing here as well
            }
          });
        }
        None => println!("Error getting the splash screen window"),
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
