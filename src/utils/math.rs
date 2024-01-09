use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

use super::structs::{Point, Shape};

const Z_OFFSET: f64 = -4.0;

pub fn calc_points_pos(
    screen_shape: &mut Shape,
    rotation: f64,
    base_shape: Shape,
    shape_size: f64,
) -> (Shape, Shape, Shape) {
    let mut projected_shape: Shape = base_shape.clone();
    let mut shape_cache = base_shape.clone();

    let itter_clone = screen_shape.points.clone();

    for (i, _point) in itter_clone.iter().enumerate() {
        screen_shape.points[i].x = base_shape.points[i].x * rad(rotation).cos()
            - base_shape.points[i].z * rad(rotation).sin();
        screen_shape.points[i].y = base_shape.points[i].y;
        screen_shape.points[i].z = base_shape.points[i].x * rad(rotation).sin()
            + base_shape.points[i].z * rad(rotation).cos()
            + Z_OFFSET;

        shape_cache.points[i].x = screen_shape.points[i].x;
        shape_cache.points[i].y = screen_shape.points[i].y;
        shape_cache.points[i].z = screen_shape.points[i].z;

        projected_shape.points[i].x =
            screen_shape.points[i].x / screen_shape.points[i].z * shape_size;
        projected_shape.points[i].y =
            screen_shape.points[i].y / screen_shape.points[i].z * shape_size;
        projected_shape.points[i].z = screen_shape.points[i].z * shape_size;
    }

    projected_shape.connections = base_shape.connections.clone();
    shape_cache.connections = Box::new([]);

    (screen_shape.clone(), projected_shape, shape_cache)
}

/*
pub fn calc_point_visibility(point: &Point, shape: &Shape) -> bool {
    // TODO:

    //visible

    true
}
*/

pub fn calc_closest_points(base_point: &Point, shape: &Shape) -> Vec<Point> {
    struct ClosestPoints {
        id: usize,
        distance: f64,
    }

    let mut distances: Vec<ClosestPoints> = Vec::new();

    if shape.points.len() < 4 {
        return Vec::new();
    }

    for point in shape.points.iter() {
        let distance = ((point.x - base_point.x).powi(2)
            + (point.y - base_point.y).powi(2)
            + (point.z - base_point.z).powi(2))
        .sqrt();

        distances.push(ClosestPoints {
            id: point.id,
            distance,
        });
    }

    distances.remove(
        distances
            .iter()
            .position(|p| p.id == base_point.id)
            .unwrap(),
    );
    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let closest_points = distances
        .iter()
        .take(3)
        .map(|p| shape.points[p.id])
        .collect();

    closest_points
}

fn rad(deg: f64) -> f64 {
    deg * std::f32::consts::PI as f64 / 180.0
}

pub fn generate_random_number(max: u32) -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_millis();

    std::thread::sleep(std::time::Duration::from_millis(1));

    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    let hash = hasher.finish();

    hash as u32 % max
}

pub fn optimize_shape(shape: &mut Shape) -> Shape {
    let mut output_points = Vec::new();

    //find any points that are less than 0.1 away from each other
    for point in shape.points.iter() {
        let mut is_close = false;

        for second_point in shape.points.iter() {
            if point.id == second_point.id {
                continue;
            }

            if (point.x - second_point.x).abs() < 0.1
                && (point.y - second_point.y).abs() < 0.1
                && (point.z - second_point.z).abs() < 0.1
            {
                is_close = true;
                break;
            }
        }

        if !is_close {
            output_points.push(*point);
        }
    }

    //shape.points = output_points;
    //mismatched types
    //expected struct `std::boxed::Box<[utils::structs::Point]>`
    //found struct `std::vec::Vec<utils::structs::Point>`

    let output_final = output_points.into_boxed_slice();

    Shape {
        points: output_final,
        connections: shape.connections.clone(),
    }
}
