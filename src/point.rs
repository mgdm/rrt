extern crate num;

use std::ops::{Add, Sub, Mul, Div};

type Float = f64;

#[derive(Serialize, Deserialize, PartialOrd, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Add<Float> for Point {
    type Output = Point;

    fn add(self, other: Float) -> Point {
        Point {x: self.x + other, y: self.y + other, z: self.z + other}
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Sub<Float> for Point {
    type Output = Point;

    fn sub(self, other: Float) -> Point {
        Point {x: self.x - other, y: self.y - other, z: self.z - other}
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }

}

impl Mul<Float> for Point {
    type Output = Point;

    fn mul(self, other: Float) -> Point {
        Point {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }
    
}

impl Div<Float> for Point {
    type Output = Point;

    fn div(self, other: Float) -> Point {
        Point {x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

impl Point {
    pub fn dot(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
