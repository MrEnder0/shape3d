use eframe::{
    egui::Ui,
    epaint::{Pos2, Stroke},
};

use super::{structs::Shape, colors::id_to_color};

pub fn render_lines(ui: &mut Ui, shape: &Shape) {
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

        let render_color = id_to_color(points[point1].id);

        ui.painter().line_segment(
            [
                Pos2 {
                    x: points[point1].x as f32 + 335.0,
                    y: points[point1].y as f32 + 110.0,
                },
                Pos2 {
                    x: points[point2].x as f32 + 335.0,
                    y: points[point2].y as f32 + 110.0,
                },
            ],
            Stroke::new(1.0, render_color),
        );
    }
}