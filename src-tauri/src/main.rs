// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, GlobalShortcutManager, Manager};

const SHORTCUT: &str = "Cmd+Shift+Space";
const WINDOW: &str = "launcher";

fn register_shortcut(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let app_handle = app.app_handle();
    let mut shortcuts = app_handle.global_shortcut_manager();
    // Only register if we haven't already assigned something to
    // this key combo
    if !shortcuts.is_registered(SHORTCUT)? {
        shortcuts.register(SHORTCUT, move || toggle(&app_handle))?;
    }

    Ok(())
}

fn toggle(app_handle: &AppHandle) {
    let win = app_handle
        .get_window(WINDOW)
        .expect("'launcher' should be defined in tauri.conf.json");

    if let Ok(true) = win.is_visible() {
        let _ = win.hide();
    } else {
        let _ = win.show();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(window: tauri::Window, name: &str) {
    println!("Hello, {}! You've been greeted from Rust!", name);
    toggle(&window.app_handle());
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            register_shortcut(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
