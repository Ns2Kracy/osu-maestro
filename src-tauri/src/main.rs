// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use osu_maestro::{logger, server};

#[tokio::main]
async fn main() {
    logger::init_tracing().await;

    let builder = tauri::Builder::default();

    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new();

    builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            specta_builder.mount_events(app);

            tokio::spawn(async move {
                server::init_server().await.expect("failed to start server");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
