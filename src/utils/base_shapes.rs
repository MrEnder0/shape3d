use super::structs::*;

pub const BASE_CUBE: Cube = Cube {
    points: [
        Point {
            x: -1.0,
            y: -1.0,
            z: 1.0,
            id: 0,
        },
        Point {
            x: 1.0,
            y: -1.0,
            z: 1.0,
            id: 1,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            id: 2,
        },
        Point {
            x: -1.0,
            y: 1.0,
            z: 1.0,
            id: 3,
        },
        Point {
            x: -1.0,
            y: -1.0,
            z: -1.0,
            id: 4,
        },
        Point {
            x: 1.0,
            y: -1.0,
            z: -1.0,
            id: 5,
        },
        Point {
            x: 1.0,
            y: 1.0,
            z: -1.0,
            id: 6,
        },
        Point {
            x: -1.0,
            y: 1.0,
            z: -1.0,
            id: 7,
        },
    ],
};
