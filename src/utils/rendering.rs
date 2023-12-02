use eframe::{egui::{Ui, self}, epaint::{Pos2, Stroke}};

use super::structs::Cube;

pub fn render_lines(ui: &mut Ui, id: usize, cube: &Cube, render_color: egui::Color32) {
    let points = cube.points.clone();

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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
                &Cube {
                    points: points.clone(),
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
    cube: &Cube,
    base_point: usize,
    child_points: Vec<usize>,
    render_color: egui::Color32,
) {
    for point in child_points {
        ui.painter().line_segment(
            [
                Pos2 {
                    x: cube.points[base_point].x as f32 + 335.0,
                    y: cube.points[base_point].y as f32 + 110.0,
                },
                Pos2 {
                    x: cube.points[point].x as f32 + 335.0,
                    y: cube.points[point].y as f32 + 110.0,
                },
            ],
            Stroke::new(1.0, render_color),
        );
    }
}