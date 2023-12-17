use eframe::{
    egui::{self, Ui},
    epaint::{FontId, Pos2, Stroke, Vec2},
};

use super::{
    colors::{id_to_color, mix_colors},
    structs::Shape,
};

pub fn render_lines(ui: &mut Ui, shape: &Shape, offset: (f32, f32)) {
    let points = shape.points.clone();

    for connection in shape.connections.iter() {
        let point1 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point1);

        let point2 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point2);

        if point1.is_none() || point2.is_none() {
            continue;
        }

        let point1 = point1.unwrap().0;
        let point2 = point2.unwrap().0;

        let render_color = mix_colors(
            [
                id_to_color(points[point1].id),
                id_to_color(points[point2].id),
            ]
            .to_vec(),
        );

        ui.painter().line_segment(
            [
                Pos2 {
                    x: points[point1].x as f32 + offset.0,
                    y: points[point1].y as f32 + offset.1,
                },
                Pos2 {
                    x: points[point2].x as f32 + offset.0,
                    y: points[point2].y as f32 + offset.1,
                },
            ],
            Stroke::new(1.0, render_color),
        );
    }
}

pub fn dynamic_render_lines(ui: &mut Ui, shape: &Shape, offset: (f32, f32), shape_size: f32) {
    for base_point in shape.points.iter() {
        let closest_points = super::math::calc_closest_points(base_point, shape);

        if closest_points.is_empty() {
            // Do text at center of screen
            ui.painter().text(
                Pos2 {
                    x: ui.available_width() / 2.0,
                    y: ui.available_height() / 2.0,
                },
                egui::Align2::CENTER_CENTER,
                "This rendering method does not support shapes with less than 4 points",
                FontId::default(),
                id_to_color(base_point.id),
            );
        }

        let points = shape.points.clone();
        let starting_point = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == base_point.id);

        for point in closest_points.iter() {
            if point.id == base_point.id {
                continue;
            }

            let render_color_one = id_to_color(point.id);
            let render_color_two = id_to_color(base_point.id);

            ui.painter().line_segment(
                [
                    Pos2 {
                        x: ((shape.points[starting_point.unwrap().0].x
                            / shape.points[starting_point.unwrap().0].z)
                            as f32
                            * shape_size)
                            + offset.0,
                        y: ((shape.points[starting_point.unwrap().0].y
                            / shape.points[starting_point.unwrap().0].z)
                            as f32
                            * shape_size)
                            + offset.1,
                    },
                    Pos2 {
                        x: ((point.x / point.z) as f32 * shape_size) + offset.0,
                        y: ((point.y / point.z) as f32 * shape_size) + offset.1,
                    },
                ],
                Stroke::new(
                    1.0,
                    mix_colors([render_color_one, render_color_two].to_vec()),
                ),
            );
        }
    }
}

/// Experiment #1
pub fn render_sides(ui: &mut Ui, shape: &Shape, offset: (f32, f32)) {
    let points = shape.points.clone();

    for connection in shape.connections.iter() {
        let point1 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point1)
            .unwrap()
            .0;

        let point2 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point2)
            .unwrap()
            .0;

        let render_color = mix_colors(
            [
                id_to_color(points[point1].id),
                id_to_color(points[point2].id),
            ]
            .to_vec(),
        );

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                Pos2 {
                    x: points[point1].x as f32 + offset.0,
                    y: points[point1].y as f32 + offset.1,
                },
                Vec2 {
                    x: points[point2].x as f32 - points[point1].x as f32 + 10.0,
                    y: points[point2].y as f32 - points[point1].y as f32 + 10.0,
                },
            ),
            10.0,
            render_color,
        );
    }
}
