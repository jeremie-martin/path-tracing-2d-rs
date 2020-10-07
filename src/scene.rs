use crate::light::Light;
use crate::object::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub ray_count: i64,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}
