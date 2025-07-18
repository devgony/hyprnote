mod commands;
mod error;
mod ext;
mod license;
mod machine;
mod store;

pub use error::*;
pub use ext::*;
pub use store::*;

const PLUGIN_NAME: &str = "membership";

fn make_specta_builder<R: tauri::Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::<R>::new()
        .plugin_name(PLUGIN_NAME)
        .commands(tauri_specta::collect_commands![
            commands::check::<tauri::Wry>,
        ])
        .error_handling(tauri_specta::ErrorHandlingMode::Throw)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    let specta_builder = make_specta_builder();

    tauri::plugin::Builder::new(PLUGIN_NAME)
        .invoke_handler(specta_builder.invoke_handler())
        .setup(|_app, _api| {
            // https://github.com/ahonn/keygen-rs/blob/c02516c/packages/tauri-plugin-keygen-rs2/src/lib.rs#L116
            let config = keygen_rs::config::KeygenConfig {
                api_url: "https://api.keygen.sh".to_string(),
                // https://keygen.sh/docs/api/versioning
                api_version: "1.8".to_string(),
                api_prefix: "v1".to_string(),
                account: "fastrepl".to_string(),
                product: "a5a28e7d-f1b9-4ef3-b29a-4579ecf0dab7".to_string(),
                public_key: Some("".to_string()),
                ..Default::default()
            };
            keygen_rs::config::set_config(config);

            Ok(())
        })
        .build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn export_types() {
        make_specta_builder::<tauri::Wry>()
            .export(
                specta_typescript::Typescript::default()
                    .header("// @ts-nocheck\n\n")
                    .formatter(specta_typescript::formatter::prettier)
                    .bigint(specta_typescript::BigIntExportBehavior::Number),
                "./js/bindings.gen.ts",
            )
            .unwrap()
    }

    fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
        builder
            .plugin(init())
            .plugin(tauri_plugin_store::Builder::default().build())
            .build(tauri::test::mock_context(tauri::test::noop_assets()))
            .unwrap()
    }

    #[test]
    fn test_membership() {
        let _app = create_app(tauri::test::mock_builder());
    }
}
