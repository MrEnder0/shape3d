use super::structs::{Point, Shape};

const Z_OFFSET: f64 = -4.0;

pub fn calc_points_pos(
    screen_shape: &mut Shape,
    rotation: f64,
    base_shape: Shape,
    shape_size: f64,
) -> (Shape, Shape) {
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
            screen_shape.points[i].x / screen_shape.points[i].z * shape_size;
        projected_shape.points[i].y =
            screen_shape.points[i].y / screen_shape.points[i].z * shape_size;
        projected_shape.points[i].z = screen_shape.points[i].z;
    }

    projected_shape.connections = base_shape.connections.clone();

    (screen_shape.clone(), projected_shape)
}

pub fn calc_point_visibility(point: &Point, shape: &Shape) -> bool {
    // TODO:

    //visible

    true
}

fn rad(deg: f64) -> f64 {
    deg * std::f32::consts::PI as f64 / 180.0
}
