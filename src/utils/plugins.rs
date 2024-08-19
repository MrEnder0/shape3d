use eframe::egui;
use libloading::Library;
use shape3d_common::Shape;

pub fn import_file_ui(loaded_lib: &Option<Library>, ctx: egui::Context, base_shape: &mut Shape) -> Option<Shape> {
    let loaded_lib = match loaded_lib {
        Some(lib) => lib,
        None => return None,
    };

    unsafe {
        let import_file_ui: libloading::Symbol<extern "C" fn(egui::Context, Shape) -> Shape> =
            match loaded_lib.get("import_file_ui".as_bytes()) {
                Ok(func) => func,
                Err(_) => {
                    return None;
                }
            };

        Some(import_file_ui(ctx, base_shape.clone()))
    }
}

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
