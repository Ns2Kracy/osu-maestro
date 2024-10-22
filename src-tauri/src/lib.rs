pub mod error;
pub mod overlay;
pub mod routes;
pub mod server;
pub mod utils;

pub async fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_devtools::init());
    }

    #[cfg(not(debug_assertions))]
    {
        utils::logger::init_tracing();
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

    builder = builder
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            specta_builder.mount_events(app);

            tokio::spawn(async move {
                server::init_server().await.expect("failed to start server");
            });

            Ok(())
        });

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, e| {
        if let tauri::RunEvent::ExitRequested { api, code, .. } = e {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    })
}
