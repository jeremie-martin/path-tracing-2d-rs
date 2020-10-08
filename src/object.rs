use crate::utils::{Point, Ray, Vec2, *};
use ncollide2d::bounding_volume::HasBoundingVolume;
use ncollide2d::query::RayCast;
use serde::{Deserialize, Serialize};

pub trait Intersectable {
    /// Returns either the distance to the closest intersection point from the origin of the ray,
    /// or None if there is no intersection
    fn intersect(&self, ray: Ray) -> f64;

    /// Returns the normal vector at point p on the object
    fn normal(&self, p: Vec2) -> Vec2;
}

pub struct SceneObject {
    geometry: Box<dyn RayCast<f64> + Sync>,
    transform: Isometry,
}

impl SceneObject {
    pub fn new<G>(geometry: Box<G>, transform: Isometry) -> SceneObject
    where
        G: 'static + Sync + RayCast<f64>,
    {
        SceneObject {
            geometry,
            transform,
        }
    }

    pub fn cast(&self, ray: &Ray) -> Option<RayIntersection> {
        #[cfg(feature = "profile")]
        let _guard = flame::start_guard("Scene Cast");
        self.geometry
            .toi_and_normal_with_ray(&self.transform, ray, 20.0, false)
    }
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
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec2,
}
