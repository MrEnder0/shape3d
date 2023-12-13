use eframe::{
    egui::{Ui, self},
    epaint::{Pos2, Stroke, Vec2},
};

use super::{structs::Shape, colors::{id_to_color, mix_colors}};

pub fn render_lines(ui: &mut Ui, shape: &Shape, offset: (f32, f32)) {
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

        let render_color = mix_colors([id_to_color(points[point1].id), id_to_color(points[point2].id)].to_vec());

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

        let render_color = mix_colors([id_to_color(points[point1].id), id_to_color(points[point2].id)].to_vec());

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                Pos2 {
                    x: points[point1].x as f32 + offset.0,
                    y: points[point1].y as f32 + offset.1,
                },
                Vec2 { x: points[point2].x as f32 - points[point1].x as f32 + 10.0, y: points[point2].y as f32 - points[point1].y as f32 + 10.0 },
            ),
            10.0,
            render_color,
        );
    }
}
