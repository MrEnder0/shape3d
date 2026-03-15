pub struct Shape {
    pub points: Box<[Point]>,
    pub connections: Box<[Connection]>,
}

#[derive(Clone, PartialEq, Copy)]
#[repr(align(32))]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: u32,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Connection {
    pub point1: u32,
    pub point2: u32,
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
    fn remove_point(&mut self, point: u32);
}

impl RemovePoint for Shape {
    fn remove_point(&mut self, point: u32) {
        self.points = self
            .points
            .clone()
            .to_vec()
            .iter()
            .enumerate()
            .filter(|(_i, p)| p.id != point)
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
