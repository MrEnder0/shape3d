mod utils;

use eframe::{
    egui,
    epaint::{Pos2, Vec2},
};
use rand::Rng;
use utils::{
    base_shapes::*,
    colors::id_to_color,
    math::calc_points_pos,
    rendering::{dynamic_render_lines, render_lines, render_sides},
    structs::*,
};

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
    rotation_speed: f64,
    rotation_direction: bool,
    color_mode: u8,
    render_mode: u8,
    base_shape: Shape,
    base_shape_index: usize,
    render_cords: bool,
    shape_offset: (f32, f32),
    shape_size: f64,
    points_cache: Shape,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen_shape: base_cube(),
            rotation: 0.0,
            rotation_speed: 0.5,
            rotation_direction: true,
            color_mode: 0,
            render_mode: 0,
            base_shape: base_cube(),
            base_shape_index: 0,
            render_cords: false,
            shape_offset: (0.0, 0.0),
            shape_size: 300.0,
            points_cache: base_cube(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut rng = rand::thread_rng();

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
                self.rotation += self.rotation_speed;

                if self.rotation >= 360.0 {
                    self.rotation = 0.0;
                }
            }
            false => {
                self.rotation -= self.rotation_speed;

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
        self.points_cache = shape_pos_calcs.2;

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        // Used to drag the cube, must be rendered before everything else so its hidden behind the cube
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.rotation)
                        .max_decimals(2)
                        .speed(0.3),
                )
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for (_i, point) in points.iter().enumerate() {
                let color = match self.color_mode {
                    0 => id_to_color(point.id),
                    1 => {
                        if point.z > -4.8 * self.shape_size {
                            egui::Color32::from_rgb(0, 255, 0)
                        } else {
                            egui::Color32::from_rgb(255, 0, 0)
                        }
                    }
                    _ => egui::Rgba::TRANSPARENT.into(),
                };

                if self.render_mode == 0 {
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

            if self.render_mode == 1 {
                render_lines(
                    ui,
                    &Shape {
                        points: points.clone(),
                        connections: connections.clone(),
                    },
                    self.shape_offset,
                );
            }

            if self.render_mode == 2 {
                dynamic_render_lines(
                    ui,
                    &Shape {
                        points: self.points_cache.points.clone(),
                        connections: Box::new([]),
                    },
                    self.shape_offset,
                    self.shape_size as f32,
                );
            }

            if self.render_mode == 3 {
                render_sides(
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
                    ui.label(format!(
                        "ID:{}, x:{}, y:{}, z:{}",
                        point.id,
                        point.x.round(),
                        point.y.round(),
                        (point.z / self.shape_size).round()
                    ));
                }
            }
        });

        // Render ui with sliders
        egui::Window::new("Settings").show(ctx, |ui| {
            ui.add(egui::Label::new(format!("Rotation: {}", self.rotation)));
            ui.add(egui::Slider::new(&mut self.rotation_speed, 0.0..=5.0).text("Rotation Speed"));
            ui.add(egui::Slider::new(&mut self.shape_size, 50.0..=1000.0).text("Shape Size"));
            ui.add(egui::Slider::new(&mut self.render_mode, 0..=3).text("Render Mode"));
            if self.render_mode == 0 {
                ui.add(egui::Slider::new(&mut self.color_mode, 0..=1).text("Color Mode"));
            }
            let base_shape_slider =
                ui.add(egui::Slider::new(&mut self.base_shape_index, 0..=2).text("Base Shape"));

            let reverse_button = ui.add(egui::Button::new("Flip Rotation"));
            let render_cords_button = ui.add(egui::Button::new("Render Cords"));

            if base_shape_slider.changed() {
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
            }

            if reverse_button.clicked() {
                self.rotation_direction = !self.rotation_direction;
            }

            if render_cords_button.clicked() {
                self.render_cords = !self.render_cords;
            }
        });

        let mut points_to_remove: Vec<usize> = Vec::new();

        egui::Window::new("Base Shape Modifier").show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 50.0;
            ui.set_width(200.0);

            for (i, point) in self.base_shape.points.iter_mut().enumerate() {
                ui.colored_label(id_to_color(point.id), format!("Point {}", i));
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(
                        egui::Slider::new(&mut point.x, -1.0..=1.0)
                            .drag_value_speed(0.001)
                            .show_value(false),
                    )
                    .on_hover_text(point.x.to_string());
                    ui.label("Y:");
                    ui.add(
                        egui::Slider::new(&mut point.y, -1.0..=1.0)
                            .drag_value_speed(0.001)
                            .show_value(false),
                    )
                    .on_hover_text(point.y.to_string());
                    ui.label("Z:");
                    ui.add(
                        egui::Slider::new(&mut point.z, -1.0..=1.0)
                            .drag_value_speed(0.001)
                            .show_value(false),
                    )
                    .on_hover_text(point.z.to_string());
                    if ui.add(egui::Button::new("Remove")).clicked() {
                        points_to_remove.push(i);
                    }
                });

                ui.separator();
            }

            if ui.add(egui::Button::new("Add Point")).clicked() {
                self.base_shape.add_point(Point {
                    x: rng.gen_range(-1.0..=1.0),
                    y: rng.gen_range(-1.0..=1.0),
                    z: rng.gen_range(-1.0..=1.0),
                    id: self.base_shape.points.len(),
                });
                self.screen_shape.add_point(Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: self.base_shape.points.len(),
                });
            }
        });

        for point in points_to_remove.iter() {
            self.base_shape.remove_point(*point);
            self.screen_shape.remove_point(*point);
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
        ctx.request_repaint()
    }
}
