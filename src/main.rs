mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Stroke, Vec2},
};
use std::sync::Mutex;
use utils::{base_shapes::*, structs::*};

static SCREEN_CUBE: Mutex<Cube> = Mutex::new(Cube {
    points: [
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            id: 0,
        },
        Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            id: 1,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            id: 2,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 1.0,
            id: 3,
        },
        Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            id: 4,
        },
        Point {
            x: 1.0,
            y: 0.0,
            z: 1.0,
            id: 5,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            id: 6,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            id: 7,
        },
    ],
});

const Z_OFFSET: f64 = -4.0;
const CUBE_SIZE: f64 = 120.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("3D Cube", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    rotation: f64,
    rotation_direction: bool,
    color_mode: u8,
    render_mode: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            rotation: 0.0,
            rotation_direction: true,
            color_mode: 0,
            render_mode: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.rotation_direction {
            true => {
                self.rotation += 0.5;

                if self.rotation >= 360.0 {
                    self.rotation = 0.0;
                }
            }
            false => {
                self.rotation -= 0.5;

                if self.rotation <= 0.0 {
                    self.rotation = 360.0;
                }
            }
        }

        let mut points = calc_points(self.rotation).points;

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        egui::CentralPanel::default().show(ctx, |ui| {
            for (_i, point) in points.iter().enumerate() {
                let color = match self.color_mode {
                    0 => match point.id {
                        0 => egui::Color32::from_rgb(255, 0, 0),
                        1 => egui::Color32::from_rgb(0, 255, 0),
                        2 => egui::Color32::from_rgb(0, 0, 255),
                        3 => egui::Color32::from_rgb(255, 255, 0),
                        4 => egui::Color32::from_rgb(255, 0, 255),
                        5 => egui::Color32::from_rgb(0, 255, 255),
                        6 => egui::Color32::from_rgb(255, 255, 255),
                        7 => egui::Color32::from_rgb(0, 0, 0),
                        _ => egui::Color32::from_rgb(0, 0, 0),
                    },
                    1 => {
                        if point.z > -4.8 {
                            egui::Color32::from_rgb(0, 255, 0)
                        } else {
                            egui::Color32::from_rgb(255, 0, 0)
                        }
                    }
                    _ => egui::Color32::from_rgb(0, 0, 0),
                };

                match self.render_mode {
                    0 => {
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                Pos2 {
                                    x: point.x as f32 + 330.0,
                                    y: point.y as f32 + 105.0,
                                },
                                Vec2 { x: 10.0, y: 10.0 },
                            ),
                            10.0,
                            color,
                        );
                    }
                    1 => {
                        let point1 = points
                            .iter()
                            .enumerate()
                            .find(|(_i, p)| p.id == point.id)
                            .unwrap()
                            .0;
                        let point2 = match points
                            .iter()
                            .enumerate()
                            .find(|(_i, p)| p.id == point.id + 1)
                        {
                            Some(i) => i.0,
                            None => {
                                points
                                    .iter()
                                    .enumerate()
                                    .find(|(_i, p)| p.id == 0)
                                    .unwrap()
                                    .0
                            }
                        };

                        ui.painter().line_segment(
                            [
                                Pos2 {
                                    x: points[point1].x as f32 + 330.0,
                                    y: points[point1].y as f32 + 105.0,
                                },
                                Pos2 {
                                    x: points[point2].x as f32 + 330.0,
                                    y: points[point2].y as f32 + 105.0,
                                },
                            ],
                            Stroke::new(1.0, color),
                        );
                    }
                    2 => {
                        let point1 = points
                            .iter()
                            .enumerate()
                            .find(|(_i, p)| p.id == point.id)
                            .unwrap()
                            .0;
                        let point2 = match points
                            .iter()
                            .enumerate()
                            .find(|(_i, p)| p.id == point.id + 1)
                        {
                            Some(i) => i.0,
                            None => {
                                points
                                    .iter()
                                    .enumerate()
                                    .find(|(_i, p)| p.id == 0)
                                    .unwrap()
                                    .0
                            }
                        };

                        ui.painter().line_segment(
                            [
                                Pos2 {
                                    x: points[point1].x as f32 + 330.0,
                                    y: points[point1].y as f32 + 105.0,
                                },
                                Pos2 {
                                    x: points[point2].x as f32 + 330.0,
                                    y: points[point2].y as f32 + 105.0,
                                },
                            ],
                            Stroke::new(1.0, color),
                        );

                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                Pos2 {
                                    x: point.x as f32 + 330.0,
                                    y: point.y as f32 + 105.0,
                                },
                                Vec2 { x: 10.0, y: 10.0 },
                            ),
                            10.0,
                            color,
                        );
                    }
                    _ => {}
                }
            }
        });

        // Render ui with sliders
        egui::Window::new("3D Cube").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0).text("Rotation"));
            let reverse_button = ui.add(egui::Button::new("Flip Rotation"));
            ui.add(egui::Slider::new(&mut self.color_mode, 0..=1).text("Color Mode"));
            ui.add(egui::Slider::new(&mut self.render_mode, 0..=2).text("Render Mode"));

            if reverse_button.clicked() {
                self.rotation_direction = !self.rotation_direction;
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(10));
        ctx.request_repaint()
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
