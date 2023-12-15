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

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.connections == other.connections
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.id == other.id
    }
}

impl Copy for Point {}

impl Clone for Connection {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2
    }
}

impl Copy for Connection {}