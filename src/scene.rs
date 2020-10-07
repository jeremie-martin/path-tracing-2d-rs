extern crate serde;
extern crate serde_json;

use crate::light::Light;
use crate::object::{Object, SceneObject};
use crate::utils::*;
use line_drawing::XiaolinWu;
use serde::{Deserialize, Serialize};
use std::fs::File;

use ncollide2d::shape::{Ball, Segment};

// TODO: Properly deserialize objects
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneParsing {
    pub ray_count: i64,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

pub struct Scene {
    pub ray_count: i64,
    pub lights: Vec<Light>,
    pub objects: Vec<SceneObject>,
    pub screen: Vec<f64>,
}

impl Scene {
    pub fn new(path: &str) -> Scene {
        let scene_file = File::open(path).expect("File not found");
        let scene: SceneParsing = serde_json::from_reader(&scene_file).unwrap();
        let mut objects: Vec<SceneObject> = Vec::new();

        for obj in scene.objects.iter() {
            let tmp = match obj {
                Object::Sphere(s) => SceneObject::new(
                    Box::new(Ball::new(s.radius)),
                    Isometry::new(s.center.clone(), 0.0),
                ),
                Object::Segment(s) => SceneObject::new(
                    Box::new(Segment::new(s.p1, s.p2)),
                    Isometry::new(Vec2::new(0.0, 0.0), 0.0),
                ),
            };

            objects.push(tmp);
            println!("aaa {}", objects.len());
        }

        Scene {
            ray_count: scene.ray_count,
            lights: scene.lights,
            objects,
        }
    }

    pub fn getClosest(&self, ray: &Ray) -> Option<RayIntersection> {
        self.objects[1..].iter().fold(None, |closest, obj| {
            let inter = obj.cast(&ray);
            if closest.is_none() {
                return inter;
            }
            match (closest, inter) {
                (Some(clo), Some(int)) => {
                    if int.toi < clo.toi {
                        inter
                    } else {
                        closest
                    }
                }
                (None, _) => inter,
                (_, None) => closest,
            }
        });
    }

    pub fn trace(&self) {
        for light in self.lights.iter() {
            let ray = light.get_ray();

            let intersection = getClosest(&ray);
            println!("r.p {}, r.d {}", ray.origin, ray.dir);
            if let Some(inter) = intersection {
                for ((x, y), value) in XiaolinWu::<f32, i8>::new(ray.origin, inter) {}
            }
        }
    }
}
