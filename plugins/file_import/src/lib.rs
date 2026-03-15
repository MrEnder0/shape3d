use eframe::egui;
use shape3d_common::{Point, Shape};
use std::path::PathBuf;

use rfd::FileDialog;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn import_file_ui(ctx: egui::Context, base_shape: Shape) -> Shape {
    let mut return_value = base_shape.clone();

    egui::Window::new("File Management").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Import File").clicked() {
                let path = import_file();
                if let Some(path) = path {
                    let mut points: Vec<Point> = Vec::new();
                    let file = std::fs::read_to_string(&path).unwrap();

                    match path.extension() {
                        Some(ext) if ext == "pc" => {
                            for (id, line) in file.lines().enumerate() {
                                let mut split = line.split_whitespace();
                                let x = split.next().unwrap().parse::<f64>().unwrap();
                                let y = split.next().unwrap().parse::<f64>().unwrap();
                                let z = split.next().unwrap().parse::<f64>().unwrap();

                                points.push(Point {
                                    id: id as u32,
                                    x: x.into(),
                                    y: y.into(),
                                    z: z.into(),
                                });
                            }
                        }
                        Some(ext) if ext == "obj" => {
                            for (id, line) in file.lines().enumerate() {
                                if line.starts_with("v ") {
                                    let mut split = line[2..].split_whitespace();
                                    let x = split.next().unwrap().parse::<f64>().unwrap();
                                    let y = split.next().unwrap().parse::<f64>().unwrap();
                                    let z = split.next().unwrap().parse::<f64>().unwrap();

                                    points.push(Point {
                                        id: id as u32,
                                        x: x.into(),
                                        y: y.into(),
                                        z: z.into(),
                                    });
                                }
                            }
                        }
                        _ => {
                            // Load logging path and log here
                            return;
                        }
                    }

                    // Always normalize the points before creating the Shape
                    points = normallize_model(points);

                    return_value = Shape {
                        points: points.into(),
                        connections: Vec::new().into(),
                    };
                }
            }
            let export_file_button = ui.button("Export File");
            if export_file_button.clicked() {
                let path = export_file();
                if let Some(path) = path {
                    let mut save = String::new();

                    for point in base_shape.points.iter() {
                        save.push_str(&format!("{} {} {}\n", point.x, point.y, point.z));
                    }

                    match std::fs::write(path, save) {
                        Err(_) => {
                            // Load logging path and log here
                        }
                        _ => {}
                    }
                }
            }
        });

        ui.add(egui::Separator::default());

        ui.label(format!("Plugin Version: {}", VERSION));
    });

    return_value
}

fn import_file() -> Option<PathBuf> {
    let current_path = std::env::current_dir().unwrap();

    FileDialog::new()
        .add_filter("point cloud", &["pc", "obj"])
        .set_directory(&current_path)
        .pick_file()
}

fn export_file() -> Option<PathBuf> {
    let current_path = std::env::current_dir().unwrap();

    FileDialog::new()
        .add_filter("point cloud", &["pc"])
        .set_directory(&current_path)
        .save_file()
}

fn normallize_model(points: Vec<Point>) -> Vec<Point> {
    // find furthest out point and then divide all points by that distance to normallize the model
    let mut furthest_distance = 0.0;
    for point in points.iter() {
        let distance = (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt();
        if distance > furthest_distance {
            furthest_distance = distance;
        }
    }

    points
        .into_iter()
        .map(|point| Point {
            id: point.id,
            x: point.x / furthest_distance,
            y: point.y / furthest_distance,
            z: point.z / furthest_distance,
        })
        .collect()
}