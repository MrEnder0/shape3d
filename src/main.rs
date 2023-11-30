mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Vec2},
};
use std::sync::Mutex;
use utils::{base_shapes::*, structs::*};

static SCREEN_CUBE: Mutex<Cube> = Mutex::new(Cube {
    points: [
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Point {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ],
});

const Z_OFFSET: f64 = -4.0;
const CUBE_SIZE: f64 = 70.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "3D Cube",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    rotation: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { rotation: 0.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.rotation += 1.0;

        let mut points = calc_points(self.rotation).points;

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, point) in points.iter().enumerate() {
                /*let color = match i {
                    0 => egui::Color32::from_rgb(255, 0, 0),
                    1 => egui::Color32::from_rgb(0, 255, 0),
                    2 => egui::Color32::from_rgb(0, 0, 255),
                    3 => egui::Color32::from_rgb(255, 255, 0),
                    4 => egui::Color32::from_rgb(255, 0, 255),
                    5 => egui::Color32::from_rgb(0, 255, 255),
                    6 => egui::Color32::from_rgb(255, 255, 255),
                    7 => egui::Color32::from_rgb(0, 0, 0),
                    _ => egui::Color32::from_rgb(0, 0, 0),
                };*/

                let color = if points[i].z > -4.8 {
                    egui::Color32::from_rgb(0, 255, 0)
                } else {
                    egui::Color32::from_rgb(255, 0, 0)
                };

                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        Pos2 {
                            x: point.x as f32 + 155.0,
                            y: point.y as f32 + 105.0,
                        },
                        Vec2 { x: 10.0, y: 10.0 },
                    ),
                    10.0,
                    color,
                );
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn calc_points(rotation: f64) -> Cube {
    let mut projected_cube: Cube = Default::default();

    let binding = &SCREEN_CUBE;

    let itter_clone = binding.lock().unwrap().points.clone();

    for (i, _point) in itter_clone.iter().enumerate() {
        let mut guard = binding.lock().unwrap();
        guard.points[i].x = BASE_CUBE.points[i].x * rad(rotation).cos()
            - BASE_CUBE.points[i].z * rad(rotation).sin();
        guard.points[i].y = BASE_CUBE.points[i].y;
        guard.points[i].z = BASE_CUBE.points[i].x * rad(rotation).sin()
            + BASE_CUBE.points[i].z * rad(rotation).cos()
            + Z_OFFSET;
        drop(guard);

        let screen_cube = SCREEN_CUBE.lock().unwrap();
        projected_cube.points[i].x = screen_cube.points[i].x / screen_cube.points[i].z * CUBE_SIZE;
        projected_cube.points[i].y = screen_cube.points[i].y / screen_cube.points[i].z * CUBE_SIZE;
        projected_cube.points[i].z = screen_cube.points[i].z;
    }

    Cube {
        points: projected_cube.points,
    }
}

fn rad(deg: f64) -> f64 {
    deg * std::f32::consts::PI as f64 / 180.0
}
