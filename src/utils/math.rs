use shape3d_common::{Point, Shape};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

const Z_OFFSET: f64 = -4.0;

pub fn calc_points_pos(
    screen_shape: &mut Shape,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    base_shape: Shape,
    shape_size: f64,
) -> (Shape, Shape, Shape) {
    let mut projected_shape: Shape = base_shape.clone();
    let mut shape_cache = base_shape.clone();

    let itter_clone = screen_shape.points.clone();

    for (i, _point) in itter_clone.iter().enumerate() {
        let point = rotate_x(&base_shape.points[i], rotation_x);
        let point = rotate_y(&point, rotation_y);
        let point = rotate_z(&point, rotation_z);

        screen_shape.points[i].x = point.x;
        screen_shape.points[i].y = point.y;
        screen_shape.points[i].z = point.z + Z_OFFSET;

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

fn rotate_x(point: &Point, angle: f64) -> Point {
    Point {
        id: point.id,
        x: point.x,
        y: point.y * rad(angle).cos() - point.z * rad(angle).sin(),
        z: point.y * rad(angle).sin() + point.z * rad(angle).cos(),
    }
}

fn rotate_y(point: &Point, angle: f64) -> Point {
    Point {
        id: point.id,
        x: point.x * rad(angle).cos() + point.z * rad(angle).sin(),
        y: point.y,
        z: -point.x * rad(angle).sin() + point.z * rad(angle).cos(),
    }
}

fn rotate_z(point: &Point, angle: f64) -> Point {
    Point {
        id: point.id,
        x: point.x * rad(angle).cos() - point.y * rad(angle).sin(),
        y: point.x * rad(angle).sin() + point.y * rad(angle).cos(),
        z: point.z,
    }
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

    if shape.points.len() < 4 {
        return Vec::new();
    }

    let mut distances: Vec<ClosestPoints> = Vec::new();

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

            if ((point.x - second_point.x).abs() < 0.1
                && (point.y - second_point.y).abs() < 0.1
                && (point.z - second_point.z).abs() < 0.1)
                || ((second_point.x - point.x).abs() < 0.1
                    && (second_point.y - point.y).abs() < 0.1
                    && (second_point.z - point.z).abs() < 0.1)
            {
                let avg_point = Point {
                    id: point.id,
                    x: (point.x + second_point.x) / 2.0,
                    y: (point.y + second_point.y) / 2.0,
                    z: (point.z + second_point.z) / 2.0,
                };

                output_points.push(avg_point);
                is_close = true;
                break;
            }
        }

        if !is_close {
            let rounded_point = Point {
                id: point.id,
                x: round_with_precision(point.x, 4),
                y: round_with_precision(point.y, 4),
                z: round_with_precision(point.z, 4),
            };

            output_points.push(rounded_point);
        }
    }

    Shape {
        points: output_points.into_boxed_slice(),
        connections: shape.connections.clone(),
    }
}

fn round_with_precision(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals);
    (x * y as f64).round() / y as f64
}
