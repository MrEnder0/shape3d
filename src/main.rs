mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Vec2},
};
use std::sync::Mutex;
use utils::{structs::*, math::calc_points, rendering::render_lines};

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
                    2 => if point.z > -4.8 {
                        egui::Color32::from_rgb(255, 255, 255)
                    } else {
                        egui::Rgba::TRANSPARENT.into()
                    },
                    _ => egui::Rgba::TRANSPARENT.into(),
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
                        render_lines(
                            ui,
                            point.id,
                            &Cube {
                                points: points.clone(),
                            },
                            color
                        );
                    }
                    2 => {
                        render_lines(
                            ui,
                            point.id,
                            &Cube {
                                points: points.clone(),
                            },
                            color
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
            ui.add(egui::Slider::new(&mut self.color_mode, 0..=2).text("Color Mode"));
            ui.add(egui::Slider::new(&mut self.render_mode, 0..=2).text("Render Mode"));

            if reverse_button.clicked() {
                self.rotation_direction = !self.rotation_direction;
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(10));
        ctx.request_repaint()
    }
}
