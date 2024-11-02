use nalgebra::Vector3;

use std::{
    fmt::{Debug, Display},
    i64,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

pub struct CollisionPoint {
    pub pos: Vector3<f64>,
    pub time: f64,
}

pub struct TestArea {
    pub x_range: (i64, i64),
    pub y_range: (i64, i64),
}

pub trait TestAreaInBounds {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool;
}

impl TestArea {
    pub fn new(x_range: (i64, i64), y_range: (i64, i64)) -> Self {
        TestArea { x_range, y_range }
    }
    pub fn is_in_bounds<T: TestAreaInBounds>(&self, obj: &T) -> bool {
        obj.is_in_bounds(&self)
    }
}

#[derive(Debug, Clone)]
pub struct Vec3<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::str::FromStr
        + Debug
        + Display
        + Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::str::FromStr
        + std::cmp::PartialOrd
        + Debug
        + Display
        + Clone
{
}

impl PartialEq for Vec3<i64> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl TestAreaInBounds for Vector3<f64> {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool {
        self.x >= test_area.x_range.0 as f64
            && self.x <= test_area.x_range.1 as f64
            && self.y >= test_area.y_range.0 as f64
            && self.y <= test_area.y_range.1 as f64
    }
}

impl From<(i64, i64, i64)> for Vec3<i64> {
    fn from(t: (i64, i64, i64)) -> Self {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone
        + Display
        + Debug
        + FromStr,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone(),
            z: self.z.clone() + other.z.clone(),
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone
        + Display
        + Debug
        + FromStr,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x.clone() - other.x.clone(),
            y: self.y.clone() - other.y.clone(),
            z: self.z.clone() - other.z.clone(),
        }
    }
}

impl<T> Mul for Vec3<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone
        + Display
        + Debug
        + FromStr,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x.clone() * other.x.clone(),
            y: self.y.clone() * other.y.clone(),
            z: self.z.clone() * other.z.clone(),
        }
    }
}

impl<T> Div for Vec3<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clone
        + Display
        + Debug
        + FromStr,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x.clone() / other.x.clone(),
            y: self.y.clone() / other.y.clone(),
            z: self.z.clone() / other.z.clone(),
        }
    }
}

#[derive(Debug)]
pub struct HailStone {
    pub pos: Vector3<f64>,
    pub vel: Vector3<f64>,
}

impl HailStone {
    pub fn new(pos: Vector3<f64>, vel: Vector3<f64>) -> Self {
        HailStone { pos, vel }
    }

    pub fn get_collision_point(&self, other: &HailStone) -> Option<Vector3<f64>> {
        if self.vel.y * other.vel.x == self.vel.x * other.vel.y {
            return None;
        }

        const EPSILON: f64 = 1e-2;
        let self_incl = self.vel.y / self.vel.x;
        let other_incl = other.vel.y / other.vel.x;
        let x = (self.pos.x * self_incl - self.pos.y - other.pos.x * other_incl + other.pos.y)
            / (self_incl - other_incl);
        let x_intercept = self.pos.y - self_incl * self.pos.x;
        let y = self_incl * x + x_intercept;

        let t1 = (x - self.pos.x) / self.vel.x;
        let t2 = (y - self.pos.y) / self.vel.y;
        let t3 = (x - other.pos.x) / other.vel.x;
        let t4 = (y - other.pos.y) / other.vel.y;

        if (t1 < 0.0 || t2 < 0.0 || t3 < 0.0 || t4 < 0.0) {
            return None;
        }
        if (t1 - t2).abs() > EPSILON || (t3 - t4).abs() > EPSILON {
            return None;
        }
        Some(Vector3::new(x, y, 0.0))
    }
}

pub fn get_hailstones(lines: &str) -> Vec<HailStone> {
    lines.lines().map(extract_info).collect()
}

impl TestAreaInBounds for HailStone {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool {
        self.pos.is_in_bounds(&test_area)
    }
}

pub fn extract_info(line: &str) -> HailStone {
    let parts = line.split(&[',', '@'][..]).collect::<Vec<&str>>();
    dbg!(&parts);
    let pos = Vector3::new(
        parts[0].trim().parse().unwrap(),
        parts[1].trim().parse().unwrap(),
        parts[2].trim().parse().unwrap(),
    );
    let vel = Vector3::new(
        parts[3].trim().parse().unwrap(),
        parts[4].trim().parse().unwrap(),
        parts[5].trim().parse().unwrap(),
    );
    HailStone::new(pos, vel)
}
