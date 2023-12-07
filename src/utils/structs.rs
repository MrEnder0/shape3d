pub struct Shape {
    pub points: Box<[Point]>,
    pub connections: Box<[Connection]>,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: usize,
}

pub struct Connection {
    pub point1: usize,
    pub point2: usize,
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape {
            points: self.points.clone(),
            connections: self.connections.clone(),
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Connection {}