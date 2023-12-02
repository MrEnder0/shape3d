use crate::{SHAPE_SIZE, Z_OFFSET};

use super::structs::Shape;

pub fn calc_points(screen_shape: &mut Shape, rotation: f64, base_shape: Shape) -> (Shape, Shape) {
    let mut projected_shape: Shape = base_shape.clone();

    let itter_clone = screen_shape.points.clone();

    for (i, _point) in itter_clone.iter().enumerate() {
        screen_shape.points[i].x = base_shape.points[i].x * rad(rotation).cos()
            - base_shape.points[i].z * rad(rotation).sin();
        screen_shape.points[i].y = base_shape.points[i].y;
        screen_shape.points[i].z = base_shape.points[i].x * rad(rotation).sin()
            + base_shape.points[i].z * rad(rotation).cos()
            + Z_OFFSET;

        projected_shape.points[i].x =
            screen_shape.points[i].x / screen_shape.points[i].z * SHAPE_SIZE;
        projected_shape.points[i].y =
            screen_shape.points[i].y / screen_shape.points[i].z * SHAPE_SIZE;
        projected_shape.points[i].z = screen_shape.points[i].z;
    }

    projected_shape.connections = base_shape.connections.clone();

    (screen_shape.clone(), projected_shape)
}

fn rad(deg: f64) -> f64 {
    deg * std::f32::consts::PI as f64 / 180.0
}
