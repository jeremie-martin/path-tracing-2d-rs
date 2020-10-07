use crate::utils::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub pos: Vec2,
    pub intensity: f64,
}
