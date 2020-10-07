extern crate serde;
extern crate serde_json;

use rayrs::scene::Scene;
use std::fs::File;

fn main() {
    println!("aaa");

    let scene_file = File::open("Examples/simple.json").expect("File not found");

    let scene: Scene = serde_json::from_reader(&scene_file).unwrap();

    println!("{:?}", scene);
}
