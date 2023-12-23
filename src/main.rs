mod utils;

use eframe::{
    egui,
    emath::Rangef,
    epaint::{Pos2, Vec2},
};
use utils::{
    base_shapes::*,
    colors::ColorCache,
    math::{calc_points_pos, generate_random_number},
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
    selected_render_mode: u8,
    base_shape: Shape,
    base_shape_index: usize,
    selected_base_shape_index: usize,
    render_cords: bool,
    shape_offset: (f32, f32),
    shape_size: f64,
    color_cache: ColorCache,
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
        let points_cache = shape_pos_calcs.2;

        points.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        // Used to drag the shape, must be rendered before everything else so its hidden behind the shape
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.rotation)
                        .max_decimals(2)
                        .speed(0.3),
                )
            });
        });

        // Detects scroll wheel input for zooming in and out
        if ctx.input(|i| i.scroll_delta.y).abs() > 0.0 {
            self.shape_size -= ctx.input(|i| i.scroll_delta.y) as f64;

            if self.shape_size < 50.0 {
                self.shape_size = 50.0;
            }

            if self.shape_size > 1500.0 {
                self.shape_size = 1500.0;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            for (_i, point) in points.iter().enumerate() {
                let color = match self.color_mode {
                    0 => ColorCache::get_color(&mut self.color_cache, point.id),
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
                                    point.y * -1.0
                                    //z_cord / self.shape_size * 100.0 - 1.0
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

        // Render ui with sliders
        egui::Window::new("Options").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Rotation", |ui| {
                    ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0).text("Rotation"));
                    ui.add(
                        egui::Slider::new(&mut self.rotation_speed, 0.0..=5.0)
                            .text("Rotation Speed"),
                    );

                    ui.separator();

                    let reverse_button = ui.add(egui::Button::new("Flip Rotation"));

                    if reverse_button.clicked() {
                        self.rotation_direction = !self.rotation_direction;
                    }
                });
                ui.label(self.rotation.to_string());
            });

            ui.menu_button("Rendering Mode", |ui| {
                let points_button = ui.add(
                    egui::SelectableLabel::new(
                        self.render_mode == 0,
                        "Points",
                    )
                );
                let pre_defined_lines = ui.add_enabled(
                    !matches!(self.base_shape_index, 3),
                    egui::SelectableLabel::new(self.render_mode == 1, "Pre-Defined Lines"),
                );
                let dynamic_lines = ui.add(
                    egui::SelectableLabel::new(
                        self.render_mode == 2,
                        "Dynamic Lines",
                    )
                );
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

            ui.add_enabled(
                matches!(self.render_mode, 0),
                egui::Slider::new(&mut self.color_mode, 0..=1).text("Point Color Mode"),
            );

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
                            self.selected_base_shape_index = 3;
                        }
                        if ui
                            .add_enabled(
                                matches!(self.render_mode, 0 | 2),
                                egui::Button::new("Remove"),
                            )
                            .clicked()
                        {
                            self.selected_base_shape_index = 4;
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

                    ui.separator();

                    ui.add_enabled_ui(
                        matches!(self.render_mode, 0 | 2),
                        |ui| {
                            ui.selectable_value(&mut self.selected_base_shape_index, 3, "Random Shape");
                        },
                    );

                    ui.selectable_value(&mut self.base_shape_index, 3, "Custom");
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
                            let times = generate_random_number(10) + 4;

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

                            self.selected_base_shape_index = 4;
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
                    self.selected_base_shape_index = 4;
                }
            });
        });

        for point in points_to_remove.iter() {
            self.base_shape.remove_point(*point);
            self.screen_shape.remove_point(*point);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
        ctx.request_repaint()
    }
}
