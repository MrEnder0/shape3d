mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Vec2},
};
use utils::{base_shapes::*, math::calc_points, rendering::render_lines, structs::*, colors::id_to_color};

const Z_OFFSET: f64 = -4.0;
const SHAPE_SIZE: f64 = 120.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("3D Shape", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    screen_shape: Shape,
    rotation: f64,
    rotation_direction: bool,
    color_mode: u8,
    render_mode: u8,
    base_shape: Shape,
    base_shape_index: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen_shape: base_cube(),
            rotation: 0.0,
            rotation_direction: true,
            color_mode: 0,
            render_mode: 0,
            base_shape: base_cube(),
            base_shape_index: 0,
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

        let shape_calcs = calc_points(
            &mut self.screen_shape,
            self.rotation,
            self.base_shape.clone(),
        );

        self.screen_shape = shape_calcs.0;
        let mut points = shape_calcs.1.points.clone();
        let connections = shape_calcs.1.connections.clone();

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        egui::CentralPanel::default().show(ctx, |ui| {
            for (_i, point) in points.iter().enumerate() {
                let color = match self.color_mode {
                    0 => {
                        id_to_color(point.id)
                    },
                    1 => {
                        if point.z > -4.8 {
                            egui::Color32::from_rgb(0, 255, 0)
                        } else {
                            egui::Color32::from_rgb(255, 0, 0)
                        }
                    }
                    2 => {
                        if point.z > -4.8 {
                            egui::Color32::from_rgb(255, 255, 255)
                        } else {
                            egui::Rgba::TRANSPARENT.into()
                        }
                    }
                    _ => egui::Rgba::TRANSPARENT.into(),
                };

                if self.render_mode == 0 || self.render_mode == 2 {
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
            }

            if self.render_mode == 1 || self.render_mode == 2 {
                render_lines(
                    ui,
                    &Shape {
                        points: points.clone(),
                        connections: connections.clone(),
                    }
                );
            }
        });

        // Render ui with sliders
        egui::Window::new("3D Shape").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0).text("Rotation"));
            let reverse_button = ui.add(egui::Button::new("Flip Rotation"));
            ui.add(egui::Slider::new(&mut self.color_mode, 0..=2).text("Color Mode"));
            ui.add(egui::Slider::new(&mut self.render_mode, 0..=2).text("Render Mode"));

            // Add slider for base shape
            ui.add(egui::Slider::new(&mut self.base_shape_index, 0..=2).text("Base Shape"));

            match self.base_shape_index {
                0 => {
                    self.base_shape = base_cube();
                    self.screen_shape = base_cube();
                }
                1 => {
                    self.base_shape = base_pyramid();
                    self.screen_shape = base_pyramid();
                }
                2 => {
                    self.base_shape = base_diamond();
                    self.screen_shape = base_diamond();
                }
                _ => {}
            }

            if reverse_button.clicked() {
                self.rotation_direction = !self.rotation_direction;
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1));
        ctx.request_repaint()
    }
}
