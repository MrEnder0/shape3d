use eframe::{
    egui::{self, Ui},
    epaint::{Pos2, Stroke},
};

use super::structs::Shape;

/*pub fn render_lines(ui: &mut Ui, id: usize, shape: &Shape, render_color: egui::Color32) {
    let points = shape.points.clone();

    let point0 = points
        .iter()
        .enumerate()
        .find(|(_i, p)| p.id == id)
        .unwrap()
        .0;

    match id {
        0 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 1)
                .unwrap()
                .0;
            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 3)
                .unwrap()
                .0;
            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 4)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        1 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 2)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 5)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 0)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        2 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 3)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 6)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 1)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        3 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 7)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 2)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 0)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        4 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 5)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 0)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 7)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        5 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 6)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 1)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 4)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        6 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 2)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 5)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 7)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        7 => {
            let point1 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 3)
                .unwrap()
                .0;

            let point2 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 6)
                .unwrap()
                .0;

            let point3 = points
                .iter()
                .enumerate()
                .find(|(_i, p)| p.id == 4)
                .unwrap()
                .0;

            render_corner(
                ui,
                &Shape {
                    points: points.clone(),
                    connections: Box::new([]),
                },
                point0,
                vec![point1, point2, point3],
                render_color,
            );
        }
        _ => {}
    }
}

fn render_corner(
    ui: &mut Ui,
    shape: &Shape,
    base_point: usize,
    child_points: Vec<usize>,
    render_color: egui::Color32,
) {
    for point in child_points {
        ui.painter().line_segment(
            [
                Pos2 {
                    x: shape.points[base_point].x as f32 + 335.0,
                    y: shape.points[base_point].y as f32 + 110.0,
                },
                Pos2 {
                    x: shape.points[point].x as f32 + 335.0,
                    y: shape.points[point].y as f32 + 110.0,
                },
            ],
            Stroke::new(1.0, render_color),
        );
    }
}
*/

pub fn render_lines(ui: &mut Ui, shape: &Shape, render_color: egui::Color32) {
    let points = shape.points.clone();

    for connection in shape.connections.iter() {
        // Find the index of the point with the id of point1
        let point1 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point1)
            .unwrap()
            .0;


        // Find the point with the id of point2
        let point2 = points
            .iter()
            .enumerate()
            .find(|(_i, p)| p.id == connection.point2)
            .unwrap()
            .0;

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