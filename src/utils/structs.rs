pub struct Cube {
    pub points: [Point; 8],
}

pub struct Cube2D {
    pub points: [Point2D; 8],
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Clone for Cube {
    fn clone(&self) -> Self {
        Cube {
            points: self.points.clone(),
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}