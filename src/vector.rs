use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct vec2 {
    pub x: f64,
    pub y: f64,
}
