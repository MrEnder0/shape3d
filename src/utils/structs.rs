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

impl Default for Shape {
    fn default() -> Self {
        Shape {
            points: Box::new([
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 1,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 2,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 3,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 4,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 5,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 6,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    id: 7,
                },
            ]),
            connections: Box::new([
                Connection {
                    point1: 0,
                    point2: 1,
                },
                Connection {
                    point1: 1,
                    point2: 2,
                },
                Connection {
                    point1: 2,
                    point2: 3,
                },
                Connection {
                    point1: 3,
                    point2: 0,
                },
                Connection {
                    point1: 4,
                    point2: 5,
                },
                Connection {
                    point1: 5,
                    point2: 6,
                },
                Connection {
                    point1: 6,
                    point2: 7,
                },
                Connection {
                    point1: 7,
                    point2: 4,
                },
                Connection {
                    point1: 0,
                    point2: 4,
                },
                Connection {
                    point1: 1,
                    point2: 5,
                },
                Connection {
                    point1: 2,
                    point2: 6,
                },
                Connection {
                    point1: 3,
                    point2: 7,
                },
            ]),
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