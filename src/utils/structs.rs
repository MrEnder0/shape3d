pub struct Shape {
    pub points: Box<[Point]>,
    pub connections: Box<[Connection]>,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: usize,
}

#[derive(Clone, PartialEq, Copy)]
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
