use eframe::{
    egui::{self, Ui},
    epaint::{Pos2, Stroke, Vec2},
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

// Partially working
pub fn generate_and_render_lines(ui: &mut Ui, shape: &Shape, offset: (f32, f32)) {
    for base_point in shape.points.iter() {
        let closest_points = super::math::calc_closest_points(base_point, shape);

        let points = shape.points.clone();
            let starting_point = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == base_point.id);

        for point in closest_points.iter() {
            if point.id == base_point.id {
                continue;
            }

            let render_color = id_to_color(point.id);

            ui.painter().line_segment(
                [
                    Pos2 {
                        x: shape.points[starting_point.unwrap().0].x as f32 + offset.0,
                        y: shape.points[starting_point.unwrap().0].y as f32 + offset.1,
                    },
                    Pos2 {
                        x: point.x as f32 + offset.0,
                        y: point.y as f32 + offset.1,
                    },
                ],
                Stroke::new(1.0, render_color),
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
