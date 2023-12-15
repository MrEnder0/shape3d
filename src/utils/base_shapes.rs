use super::structs::*;

pub fn base_cube() -> Shape {
    Shape {
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
        ]
        .into(),
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

pub fn base_pyramid() -> Shape {
    Shape {
        points: [
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                id: 0,
            },
            Point {
                x: 1.0,
                y: -1.0,
                z: 1.0,
                id: 1,
            },
            Point {
                x: -1.0,
                y: -1.0,
                z: 1.0,
                id: 2,
            },
            Point {
                x: 1.0,
                y: -1.0,
                z: -1.0,
                id: 3,
            },
            Point {
                x: -1.0,
                y: -1.0,
                z: -1.0,
                id: 4,
            },
        ]
        .into(),
        connections: Box::new([
            Connection {
                point1: 0,
                point2: 1,
            },
            Connection {
                point1: 0,
                point2: 2,
            },
            Connection {
                point1: 0,
                point2: 3,
            },
            Connection {
                point1: 0,
                point2: 4,
            },
            Connection {
                point1: 1,
                point2: 2,
            },
            Connection {
                point1: 2,
                point2: 4,
            },
            Connection {
                point1: 4,
                point2: 3,
            },
            Connection {
                point1: 3,
                point2: 1,
            },
        ]),
    }
}

pub fn base_diamond() -> Shape {
    Shape {
        points: [
            Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                id: 0,
            },
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                id: 1,
            },
            Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                id: 2,
            },
            Point {
                x: -1.0,
                y: 0.0,
                z: 0.0,
                id: 3,
            },
            Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
                id: 4,
            },
            Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
                id: 5,
            },
        ]
        .into(),
        connections: Box::new([
            Connection {
                point1: 0,
                point2: 1,
            },
            Connection {
                point1: 0,
                point2: 2,
            },
            Connection {
                point1: 0,
                point2: 3,
            },
            Connection {
                point1: 0,
                point2: 4,
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
                point2: 4,
            },
            Connection {
                point1: 4,
                point2: 1,
            },
            Connection {
                point1: 1,
                point2: 5,
            },
            Connection {
                point1: 2,
                point2: 5,
            },
            Connection {
                point1: 3,
                point2: 5,
            },
            Connection {
                point1: 4,
                point2: 5,
            },
        ]),
    }
}
