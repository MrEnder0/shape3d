#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use eframe::{
    egui::{self, Rect},
    emath::Rangef,
    epaint::{Pos2, Vec2},
};
use utils::{
    base_shapes::*,
    colors::ColorCache,
    math::{calc_points_pos, generate_random_number, optimize_shape},
    plugins::{get_available_plugins, import_file_ui, Plugin},
    rendering::{dynamic_render_lines, render_lines, render_sides},
};

use shape3d_common::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "3D Shape",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    is_startup: bool,
    plugins: Vec<Plugin>,
    screen_shape: Shape,
    rotation: (f64, f64, f64),
    rotation_volocity: (f64, f64, f64),
    max_rotation_volocity: f64,
    render_mode: u8,
    selected_render_mode: u8,
    base_shape: Shape,
    base_shape_index: usize,
    selected_base_shape_index: usize,
    render_cords: bool,
    shape_offset: (f32, f32),
    shape_size: f32,
    color_cache: ColorCache,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            is_startup: true,
            plugins: get_available_plugins(),
            screen_shape: base_cube(),
            rotation: (0.0, 0.0, 0.0),
            rotation_volocity: (1.8, 1.8, 1.8),
            max_rotation_volocity: 10.0,
            render_mode: 0,
            selected_render_mode: 0,
            base_shape: base_cube(),
            base_shape_index: 0,
            selected_base_shape_index: 0,
            render_cords: false,
            shape_offset: (0.0, 0.0),
            shape_size: 300.0,
            color_cache: ColorCache::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_startup
            && std::path::Path::new(&format!("{}autosave.pc", std::env::temp_dir().display()))
                .exists()
        {
            let file =
                std::fs::read_to_string(&format!("{}autosave.pc", std::env::temp_dir().display()))
                    .unwrap();
            let mut points: Vec<Point> = Vec::new();

            for line in file.lines() {
                let mut split = line.split_whitespace();
                let x = split.next().unwrap().parse::<f64>().unwrap();
                let y = split.next().unwrap().parse::<f64>().unwrap();
                let z = split.next().unwrap().parse::<f64>().unwrap();

                points.push(Point {
                    x,
                    y,
                    z,
                    id: points.len(),
                });
            }

            self.base_shape = Shape {
                points: points.clone().into_boxed_slice(),
                connections: Box::new([]),
            };

            self.screen_shape = Shape {
                points: points.into_boxed_slice(),
                connections: Box::new([]),
            };

            self.is_startup = false;
        }

        // Calculates the offset the shape needs to be in the center of the screen
        let window = ctx.input(|i| i.viewport().outer_rect).unwrap_or(Rect {
            min: Pos2 { x: 0.0, y: 0.0 },
            max: Pos2 { x: 0.0, y: 0.0 },
        });
        let window_size = (window.max.x - window.min.x, window.max.y - window.min.y);

        self.shape_offset = (
            (window_size.0 / 2.0) - (self.shape_size / 2.0) + self.shape_size / 2.0,
            (window_size.1 / 2.0) - (self.shape_size / 2.0) + self.shape_size / 2.0,
        );

        // Detects scroll wheel input for zooming in and out
        if ctx.input(|i| i.raw_scroll_delta.y).abs() > 0.0 {
            self.shape_size -= ctx.input(|i| i.raw_scroll_delta.y);

            self.shape_size = self.shape_size.clamp(50.0, 1500.0);
        }

        handle_rotation_input(ctx, &mut self.rotation_volocity);

        self.rotation_volocity.0 = self.rotation_volocity.0.clamp(
            self.max_rotation_volocity * -1.0,
            self.max_rotation_volocity,
        );
        self.rotation_volocity.1 = self.rotation_volocity.1.clamp(
            self.max_rotation_volocity * -1.0,
            self.max_rotation_volocity,
        );
        self.rotation_volocity.2 = self.rotation_volocity.2.clamp(
            self.max_rotation_volocity * -1.0,
            self.max_rotation_volocity,
        );

        self.rotation = (
            self.rotation.0 + self.rotation_volocity.0,
            self.rotation.1 + self.rotation_volocity.1,
            self.rotation.2 + self.rotation_volocity.2,
        );

        self.rotation_volocity.0 *= 0.98;
        self.rotation_volocity.1 *= 0.98;
        self.rotation_volocity.2 *= 0.98;

        let normalized_rotation = (
            self.rotation.0 % 360.0,
            self.rotation.1 % 360.0,
            self.rotation.2 % 360.0,
        );

        let shape_pos_calcs = calc_points_pos(
            &mut self.screen_shape,
            normalized_rotation,
            self.base_shape.clone(),
            self.shape_size.into(),
        );

        self.screen_shape = shape_pos_calcs.0;
        let mut points = shape_pos_calcs.1.points.clone();
        let connections = shape_pos_calcs.1.connections.clone();
        let points_cache = shape_pos_calcs.2;

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        egui::CentralPanel::default().show(ctx, |ui| {
            for point in points.iter() {
                let color = ColorCache::get_color(&mut self.color_cache, point.id);

                // Rendering mode 0 (Points)
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

                if self.render_cords {
                    ui.allocate_ui_at_rect(
                        egui::Rect::from_min_size(
                            Pos2 {
                                x: point.x as f32 + self.shape_offset.0 + 8.0,
                                y: point.y as f32 + self.shape_offset.1 + 8.0,
                            },
                            Vec2 { x: 100.0, y: 10.0 },
                        ),
                        |ui| {
                            ui.colored_label(
                                color,
                                format!(
                                    "X: {:.3}\nY: {:.3}",
                                    point.x,
                                    point.y * -1.0 //z_cord / self.shape_size * 100.0 - 1.0
                                ),
                            );
                        },
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
                    self.color_cache.copy(),
                    self.shape_offset,
                );
            }

            if self.render_mode == 2 {
                dynamic_render_lines(
                    ui,
                    &Shape {
                        points: points_cache.points.clone(),
                        connections: Box::new([]),
                    },
                    self.color_cache.copy(),
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
                    self.color_cache.copy(),
                    self.shape_offset,
                );
            }
        });

        egui::Window::new("Options").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Rotation", |ui| {
                    ui.add(egui::Label::new(format!("X: {:.2}", normalized_rotation.0)));
                    ui.add(
                        egui::DragValue::new(&mut self.rotation.0)
                            .speed(0.1)
                            .max_decimals(2)
                            .prefix("Absolute rotation: "),
                    );
                    ui.add(egui::Label::new(format!("Y: {:.2}", normalized_rotation.1)));
                    ui.add(
                        egui::DragValue::new(&mut self.rotation.1)
                            .speed(0.1)
                            .max_decimals(2)
                            .prefix("Absolute rotation: "),
                    );
                    ui.add(egui::Label::new(format!("Z: {:.2}", normalized_rotation.2)));
                    ui.add(
                        egui::DragValue::new(&mut self.rotation.2)
                            .speed(0.1)
                            .max_decimals(2)
                            .prefix("Absolute rotation: "),
                    );

                    ui.separator();

                    ui.add(
                        egui::Slider::new(&mut self.max_rotation_volocity, 1.0..=50.0)
                            .text("Max Volocity"),
                    );
                });
            });

            ui.menu_button("Rendering Mode", |ui| {
                let points_button =
                    ui.add(egui::SelectableLabel::new(self.render_mode == 0, "Points"));
                let pre_defined_lines = ui.add_enabled(
                    !matches!(self.base_shape_index, 3),
                    egui::SelectableLabel::new(self.render_mode == 1, "Pre-Defined Lines"),
                );
                let dynamic_lines = ui.add(egui::SelectableLabel::new(
                    self.render_mode == 2,
                    "Dynamic Lines",
                ));
                let experiment_one_button = ui.add_enabled(
                    !matches!(self.base_shape_index, 3),
                    egui::SelectableLabel::new(self.render_mode == 3, "Experiment 1"),
                );

                if points_button.clicked() {
                    self.render_mode = 0;
                    self.selected_render_mode = 0;
                }

                if pre_defined_lines.clicked() {
                    self.render_mode = 1;
                    self.selected_render_mode = 1;
                }

                if dynamic_lines.clicked() {
                    self.render_mode = 2;
                    self.selected_render_mode = 2;
                }

                if experiment_one_button.clicked() {
                    self.render_mode = 3;
                    self.selected_render_mode = 3;
                }
            });

            ui.add(egui::Slider::new(&mut self.shape_size, 50.0..=1500.0).text("Shape Size"));

            let render_cords_button = ui.add(egui::Button::new("Render Cords"));
            let reset_color_cache = ui.add(egui::Button::new("Reset Color Cache"));

            if render_cords_button.clicked() {
                self.render_cords = !self.render_cords;
            }

            if reset_color_cache.clicked() {
                self.color_cache = ColorCache::new();
            }
        });

        let mut points_to_remove: Vec<usize> = Vec::new();

        egui::Window::new("Shape Modifier").show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 50.0;
            ui.set_width(200.0);

            ui.set_height_range(Rangef::new(
                ui.available_height() / 3.0,
                ui.available_height() / 1.5,
            ));

            egui::ScrollArea::vertical().show(ui, |ui| {
                let highest_point_id = self.base_shape.points.iter().fold(0, |acc, point| {
                    if point.id > acc {
                        point.id
                    } else {
                        acc
                    }
                });

                for (i, point) in self.base_shape.points.iter_mut().enumerate() {
                    ui.colored_label(
                        ColorCache::get_color(&mut self.color_cache, point.id),
                        format!("Point {}", i),
                    );
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        let x_slider = ui
                            .add(
                                egui::Slider::new(&mut point.x, -1.0..=1.0)
                                    .drag_value_speed(0.001)
                                    .show_value(false),
                            )
                            .on_hover_text(point.x.to_string());
                        ui.label("Y:");
                        let y_slider = ui
                            .add(
                                egui::Slider::new(&mut point.y, -1.0..=1.0)
                                    .drag_value_speed(0.001)
                                    .show_value(false),
                            )
                            .on_hover_text(point.y.to_string());
                        ui.label("Z:");
                        let z_slider = ui
                            .add(
                                egui::Slider::new(&mut point.z, -1.0..=1.0)
                                    .drag_value_speed(0.001)
                                    .show_value(false),
                            )
                            .on_hover_text(point.z.to_string());

                        if x_slider.changed() || y_slider.changed() || z_slider.changed() {
                            self.selected_base_shape_index = 5;
                        }
                        if point.id == highest_point_id {
                            if ui
                                .add_enabled(
                                    matches!(self.render_mode, 0 | 2),
                                    egui::Button::new("Remove"),
                                )
                                .clicked()
                            {
                                self.selected_base_shape_index = 5;
                                points_to_remove.push(i);
                            }
                        } else if ui
                            .add_enabled(matches!(self.render_mode, 0), egui::Button::new("Remove"))
                            .clicked()
                        {
                            self.selected_base_shape_index = 5;
                            points_to_remove.push(i);
                        }
                    });

                    ui.separator();
                }
            });

            ui.horizontal(|ui| {
                ui.menu_button("Base Shape", |ui| {
                    ui.selectable_value(&mut self.selected_base_shape_index, 0, "Cube");
                    ui.selectable_value(&mut self.selected_base_shape_index, 1, "Pyramid");
                    ui.selectable_value(&mut self.selected_base_shape_index, 2, "Diamond");
                    ui.selectable_value(&mut self.selected_base_shape_index, 3, "Tetrahedron");

                    ui.separator();

                    ui.add_enabled_ui(matches!(self.render_mode, 0 | 2), |ui| {
                        ui.selectable_value(&mut self.selected_base_shape_index, 4, "Random Shape");
                    });

                    ui.selectable_value(&mut self.base_shape_index, 4, "Custom");
                });

                if self.selected_base_shape_index != self.base_shape_index {
                    match self.selected_base_shape_index {
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
                        3 => {
                            self.base_shape = base_tetrahedron();
                            self.screen_shape = base_tetrahedron();
                        }
                        4 => {
                            let times = generate_random_number(6) + 4;

                            self.base_shape = Shape {
                                points: Box::new([]),
                                connections: Box::new([]),
                            };

                            self.screen_shape = Shape {
                                points: Box::new([]),
                                connections: Box::new([]),
                            };

                            for _i in 0..times {
                                self.base_shape.add_point(Point {
                                    x: generate_random_number(200) as f64 * 0.01 - 1.0,
                                    y: generate_random_number(200) as f64 * 0.01 - 1.0,
                                    z: generate_random_number(200) as f64 * 0.01 - 1.0,
                                    id: self.base_shape.points.len(),
                                });
                                self.screen_shape.add_point(Point {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 0.0,
                                    id: self.base_shape.points.len(),
                                });
                            }

                            self.selected_base_shape_index = 5;
                        }
                        _ => {}
                    }
                    self.base_shape_index = self.selected_base_shape_index;
                }

                if ui
                    .add_enabled(
                        matches!(self.render_mode, 0 | 2),
                        egui::Button::new("Add Point"),
                    )
                    .clicked()
                {
                    self.base_shape.add_point(Point {
                        x: generate_random_number(200) as f64 * 0.01 - 1.0,
                        y: generate_random_number(200) as f64 * 0.01 - 1.0,
                        z: generate_random_number(200) as f64 * 0.01 - 1.0,
                        id: self.base_shape.points.len(),
                    });
                    self.screen_shape.add_point(Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        id: self.base_shape.points.len(),
                    });
                    self.selected_base_shape_index = 5;
                }

                if ui
                    .add_enabled(
                        matches!(self.render_mode, 0 | 2),
                        egui::Button::new("WIP Optimize Shape")
                            .fill(egui::Color32::from_rgb(150, 0, 0)),
                    )
                    .clicked()
                {
                    self.base_shape = optimize_shape(&mut self.base_shape.clone());
                    self.screen_shape = optimize_shape(&mut self.screen_shape.clone());
                }
            });
        });

        // Rewriten plugin rendering
        //write code that looks threw self.plugins for a plugin struct with the name being file_import and then pass the lib to import_file_ui
        if let Some(plugin) = self
            .plugins
            .iter()
            .find(|plugin| plugin.name == "file_import")
        {
            let new_shape = import_file_ui(
                plugin.lib.clone(),
                ctx.clone(),
                &mut self.base_shape.clone(),
            );

            if let Some(shape) = new_shape {
                self.base_shape = shape.clone();
                self.screen_shape = shape.clone();
            }
        }

        for point in points_to_remove.iter() {
            self.base_shape.remove_point(*point);
            self.screen_shape.remove_point(*point);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
        ctx.request_repaint()
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let mut save = String::new();

        for point in self.base_shape.points.iter() {
            save.push_str(&format!("{} {} {}\n", point.x, point.y, point.z));
        }

        std::fs::write(
            format!("{}autosave.pc", std::env::temp_dir().display()),
            save,
        )
        .unwrap();
    }
}

fn handle_rotation_input(ctx: &egui::Context, rotation_velocity: &mut (f64, f64, f64)) {
    fn handle_rotation_key(
        ctx: &egui::Context,
        key: egui::Key,
        rotation_velocity: &mut f64,
        negative: bool,
    ) {
        if ctx.input(|i| i.key_pressed(key)) {
            match negative {
                true => *rotation_velocity -= 0.3 * (*rotation_velocity).abs() % 10.0 * 0.5 + 1.0,
                false => *rotation_velocity += 0.3 * (*rotation_velocity).abs() % 10.0 * 0.5 + 1.0,
            }
        }
    }

    handle_rotation_key(ctx, egui::Key::ArrowUp, &mut rotation_velocity.0, false);
    handle_rotation_key(ctx, egui::Key::ArrowDown, &mut rotation_velocity.0, true);
    handle_rotation_key(ctx, egui::Key::ArrowLeft, &mut rotation_velocity.1, false);
    handle_rotation_key(ctx, egui::Key::ArrowRight, &mut rotation_velocity.1, true);
    handle_rotation_key(ctx, egui::Key::Q, &mut rotation_velocity.2, false);
    handle_rotation_key(ctx, egui::Key::E, &mut rotation_velocity.2, true);
}
