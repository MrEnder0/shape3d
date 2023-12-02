use crate::{SCREEN_CUBE, Z_OFFSET, CUBE_SIZE};

use super::{structs::Cube, base_shapes::BASE_CUBE};

pub fn calc_points(rotation: f64) -> Cube {
    let mut projected_cube: Cube = Default::default();

    let binding = &SCREEN_CUBE;

    let itter_clone = binding.lock().unwrap().points.clone();

    for (i, _point) in itter_clone.iter().enumerate() {
        let mut guard = binding.lock().unwrap();
        guard.points[i].x = BASE_CUBE.points[i].x * rad(rotation).cos()
            - BASE_CUBE.points[i].z * rad(rotation).sin();
        guard.points[i].y = BASE_CUBE.points[i].y;
        guard.points[i].z = BASE_CUBE.points[i].x * rad(rotation).sin()
            + BASE_CUBE.points[i].z * rad(rotation).cos()
            + Z_OFFSET;
        drop(guard);

        let screen_cube = SCREEN_CUBE.lock().unwrap();
        projected_cube.points[i].x = screen_cube.points[i].x / screen_cube.points[i].z * CUBE_SIZE;
        projected_cube.points[i].y = screen_cube.points[i].y / screen_cube.points[i].z * CUBE_SIZE;
        projected_cube.points[i].z = screen_cube.points[i].z;
    }

    Cube {
        points: projected_cube.points,
    }
}

fn rad(deg: f64) -> f64 {
    deg * std::f32::consts::PI as f64 / 180.0
}
