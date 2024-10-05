// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use osu_maestro::server;

#[tokio::main]
async fn main() {
    #[cfg(not(debug_assertions))]
    {
        use osu_maestro::logger;
        logger::init_tracing();
    }

    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_devtools::init())
    }

    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new();

    #[cfg(all(debug_assertions, not(mobile)))]
    specta_builder
        .export(
            specta_typescript::Typescript::default()
                .formatter(specta_typescript::formatter::prettier),
            "../src/bindings.ts",
        )
        .expect("failed to export typescript bindings");

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
        .expect("error while running osu maestro");
}
