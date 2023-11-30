pub struct Cube {
    pub points: [Point; 8],
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
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
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
