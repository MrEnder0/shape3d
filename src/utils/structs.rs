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

pub trait RemovePoint {
    fn remove_point(&mut self, point: usize);
}

impl RemovePoint for Shape {
    fn remove_point(&mut self, point: usize) {
        self.points = self
            .points
            .clone()
            .to_vec()
            .iter()
            .enumerate()
            .filter(|(i, _p)| *i != point)
            .map(|(_i, p)| *p)
            .collect::<Box<[Point]>>();

        self.connections = self
            .connections
            .clone()
            .to_vec()
            .iter()
            .filter(|c| c.point1 != point && c.point2 != point)
            .copied()
            .collect::<Box<[Connection]>>();
    }
}

pub trait AddPoint {
    fn add_point(&mut self, point: Point);
}

impl AddPoint for Shape {
    fn add_point(&mut self, point: Point) {
        self.points = self
            .points
            .clone()
            .to_vec()
            .iter()
            .chain([point].iter())
            .copied()
            .collect::<Box<[Point]>>();
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
