use super::structs::*;

pub const BASE_CUBE: Cube = Cube {
    points: [
        Point {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
        Point {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Point {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Point {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
    ],
};
