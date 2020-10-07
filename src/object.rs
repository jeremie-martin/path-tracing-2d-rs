use crate::ray::Ray;
use crate::vector::vec2;
use serde::{Deserialize, Serialize};

pub trait Intersectable {
    /// Returns either the distance to the closest intersection point from the origin of the ray,
    /// or None if there is no intersection
    fn intersect(&self, ray: Ray) -> f64;

    /// Returns the normal vector at point p on the object
    fn normal(&self, p: vec2) -> vec2;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "segment")]
    Segment(Segment),
    #[serde(rename = "sphere")]
    Sphere(Sphere),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub p1: vec2,
    pub p2: vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
    pub center: vec2,
}