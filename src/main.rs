mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Vec2},
};
use utils::{base_shapes::*, math::calc_points_pos, rendering::render_lines, structs::*, colors::id_to_color};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
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
    render_cords: bool,
    shape_offset: (f32, f32),
    shape_size: f64,
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
            render_cords: false,
            shape_offset: (0.0, 0.0),
            shape_size: 120.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculates the offset the shape needs to be in the center of the screen
        let window = ctx.input(|i| i.viewport().outer_rect).unwrap();
        let window_size = (window.max.x - window.min.x, window.max.y - window.min.y);

        self.shape_offset = (
            (window_size.0 / 2.0) - (self.shape_size / 2.0) as f32 + self.shape_size as f32 / 2.0,
            (window_size.1 / 2.0) - (self.shape_size / 2.0) as f32 + self.shape_size as f32 / 2.0,
        );

        // Updates the shape
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

        let shape_pos_calcs = calc_points_pos(
            &mut self.screen_shape,
            self.rotation,
            self.base_shape.clone(),
            self.shape_size,
        );

        self.screen_shape = shape_pos_calcs.0;
        let mut points = shape_pos_calcs.1.points.clone();
        let connections = shape_pos_calcs.1.connections.clone();

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
                                    x: point.x as f32 + self.shape_offset.0 - 5.0,
                                    y: point.y as f32 + self.shape_offset.1 - 5.0,
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
                    },
                    self.shape_offset,
                );
            }

            if self.render_cords {
                ui.label("Note: These are transformed cords with original z-axis values");

                let mut points_clone = points.clone();
                points_clone.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

                for point in points_clone.iter() {
                    ui.label(format!("ID:{}, x:{}, y:{}, z:{}", point.id, point.x.round(), point.y.round(), point.z.round()));
                }
            }
        });

        // Render ui with sliders
        egui::Window::new("3D Shape").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0).text("Rotation"));
            ui.add(egui::Slider::new(&mut self.shape_size, 40.0..=500.0).text("Shape Size"));
            let reverse_button = ui.add(egui::Button::new("Flip Rotation"));
            ui.add(egui::Slider::new(&mut self.color_mode, 0..=2).text("Color Mode"));
            ui.add(egui::Slider::new(&mut self.render_mode, 0..=2).text("Render Mode"));
            let render_cords_button = ui.add(egui::Button::new("Render Cords"));

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

            if render_cords_button.clicked() {
                self.render_cords = !self.render_cords;
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1));
        ctx.request_repaint()
    }
}
