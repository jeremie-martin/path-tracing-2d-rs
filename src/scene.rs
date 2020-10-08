extern crate serde;
extern crate serde_json;

use crate::light::Light;
use crate::object::{Object, SceneObject};
use crate::screen::Screen;
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
    pub screen: Screen,
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
                Object::Segment(s) => {
                    SceneObject::new(Box::new(Segment::new(s.p1, s.p2)), Isometry::identity())
                }
            };

            objects.push(tmp);
            println!("aaa {}", objects.len());
        }

        Scene {
            ray_count: scene.ray_count,
            lights: scene.lights,
            objects,
            screen: Screen::new(800, 800),
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
        })
    }

    pub fn trace(&mut self) {
        for _ in (0..self.ray_count) {
            for light in self.lights.iter() {
                let ray = light.get_ray();

                let intersection = self.getClosest(&ray);
                // println!("r.p {}, r.d {}", ray.origin, ray.dir);
                if let Some(inter) = intersection {
                    self.screen.draw_line(&ray.origin, &ray.point_at(inter.toi))
                }
            }
        }

        self.screen.save_img();
    }
}
