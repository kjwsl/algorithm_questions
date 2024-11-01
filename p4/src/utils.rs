use std::{
    fmt::{Debug, Display},
    i64,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

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

impl TestAreaInBounds for Vec3<i64> {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool {
        self.x >= test_area.x_range.0
            && self.x <= test_area.x_range.1
            && self.y >= test_area.y_range.0
            && self.y <= test_area.y_range.1
    }
}
impl TestAreaInBounds for Vec3<f64> {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool {
        self.x >= test_area.x_range.0 as f64
            && self.x <= test_area.x_range.1 as f64
            && self.y >= test_area.y_range.0 as f64
            && self.y <= test_area.y_range.1 as f64
    }
}

impl<T> Vec3<T>
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
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
    pub fn from_str(s: &str, delim: &str) -> Self
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut iter = s.split(delim).map(|x| x.trim().parse::<T>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Vec3 { x, y, z }
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
    pub pos: Vec3<i64>,
    pub vel: Vec3<i64>,
}

impl TestAreaInBounds for HailStone {
    fn is_in_bounds(&self, test_area: &TestArea) -> bool {
        self.pos.is_in_bounds(&test_area)
    }
}

impl HailStone {
    pub fn new(pos: Vec3<i64>, vel: Vec3<i64>) -> Self {
        HailStone { pos, vel }
    }

    pub fn get_collision_point(&self, other: &HailStone) -> Option<Vec3<f64>> {
        if self.vel.x * other.vel.y == self.vel.y * other.vel.x {
            return None;
        }
        let incline_self = self.vel.y as f64 / self.vel.x as f64;
        let offset_self = self.pos.y as f64 - incline_self * self.pos.x as f64;
        let incline_other = other.vel.y as f64 / other.vel.x as f64;
        let offset_other = other.pos.y as f64 - incline_other * other.pos.x as f64;

        let x = (offset_other - offset_self) / (incline_self - incline_other);
        if x < 0.0 {
            return None;
        }
        let y = incline_self * x + offset_self;
        let z = 0.0;

        Some(Vec3 { x, y, z })
    }
}

pub fn extract_info(line: &str) -> HailStone {
    let (left, right) = line.split_once('@').unwrap();
    let pos = Vec3::from_str(left, ",");
    let vel = Vec3::from_str(right, ",");
    HailStone::new(pos, vel)
}
