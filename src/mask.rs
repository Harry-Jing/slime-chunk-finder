use crate::coords::ChunkCoords;

use geo::Area;
use geo::BooleanOps;
use geo::{polygon, Coord, Polygon};
use std::f64::consts::PI;

const CIRCLE_NUM_POINTS: usize = 128;

fn create_square(x: f64, z: f64) -> Polygon<f64> {
    let square = polygon![
        (x: x, y: z),
        (x: x + 1.0, y: z),
        (x: x + 1.0, y: z + 1.0),
        (x: x, y: z + 1.0),
        (x: x, y: z),
    ];
    square
}

fn create_circle(center_x: f64, center_y: f64, radius: f64) -> Polygon<f64> {
    let mut points = Vec::new();
    for i in 0..CIRCLE_NUM_POINTS {
        let angle = i as f64 * 2.0 * PI / CIRCLE_NUM_POINTS as f64;
        points.push(Coord {
            x: center_x + radius * angle.cos(),
            y: center_y + radius * angle.sin(),
        });
    }
    Polygon::new(points.into(), vec![])
}

pub fn find_squares_within_circle(
    center_x: f64,
    center_y: f64,
    radius: f64,
    area_threshold: f64,
) -> Vec<ChunkCoords> {
    let mut squares_within_circle = Vec::new();

    let circle = create_circle(center_x, center_y, radius);

    for i in 0..17 {
        for j in 0..17 {
            let square = create_square(i as f64, j as f64);
            let intersection_area = circle.intersection(&square).signed_area().abs();
            if intersection_area >= area_threshold {
                squares_within_circle.push(ChunkCoords::new(i, j));
            }
        }
    }

    squares_within_circle
}
