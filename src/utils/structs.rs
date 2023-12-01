pub struct Cube {
    pub points: [Point; 8],
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: usize,
}

impl Clone for Cube {
    fn clone(&self) -> Self {
        Cube {
            points: self.points.clone(),
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            points: [
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
            ],
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
            id: self.id,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
