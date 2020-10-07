use crate::utils::{Point, Ray, Vec2};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub pos: Point,
    pub intensity: f64,
}

impl Light {
    pub fn get_ray(&self) -> Ray {
        let angle: f64 = thread_rng().gen_range(0.0, 2.0 * PI);

        Ray {
            origin: self.pos,
            dir: Vec2::new(angle.cos(), angle.sin()),
        }
    }
}
