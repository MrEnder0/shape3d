use eframe::egui;
use libloading::Library;
use shape3d_common::Shape;
use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub lib: Arc<Library>,
}

static AVAILABLE_PLUGINS: OnceLock<Vec<Plugin>> = OnceLock::new();

pub fn get_available_plugins() -> Vec<Plugin> {
    unsafe {
        AVAILABLE_PLUGINS
            .get_or_init(|| {
                let mut detected_plugins = Vec::new();

                if let Ok(lib) = Library::new("file_import") {
                    // Try to load a symbol to validate the plugin.
                    if let Ok(_) = {
                        lib.get::<extern "C" fn(egui::Context, Shape) -> Shape>(
                            "import_file_ui".as_bytes(),
                        )
                    } {
                        detected_plugins.push(Plugin {
                            name: "file_import".to_string(),
                            lib: Arc::new(lib),
                        });
                    }
                }
                detected_plugins
            })
            .to_vec()
    }
}

pub fn import_file_ui(
    loaded_lib: Arc<Library>,
    ctx: egui::Context,
    base_shape: &mut Shape,
) -> Option<Shape> {
    unsafe {
        let import_file_ui: libloading::Symbol<extern "C" fn(egui::Context, Shape) -> Shape> =
            match loaded_lib.get("import_file_ui".as_bytes()) {
                Ok(import_file_ui) => import_file_ui,
                Err(_) => return None,
            };
        Some(import_file_ui(ctx, base_shape.clone()))
    }
}

#[allow(dead_code)]
pub fn is_dynamic_plugin_valid() -> Option<Library> {
    unsafe {
        let lib = match libloading::Library::new("file_import") {
            Ok(lib) => lib,
            Err(_) => return None,
        };
        match lib.get::<extern "C" fn(egui::Context, Shape) -> Shape>("import_file_ui".as_bytes()) {
            Ok(_) => Some(lib),
            Err(_) => None,
        }
    }
}
