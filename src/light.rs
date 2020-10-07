use crate::vector::vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    #[serde(flatten)]
    pub pos: vec2,
    pub intensity: f64,
}
