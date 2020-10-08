use rayrs::scene::Scene;

fn main() {
    let mut scene = Scene::new("Examples/simple.json");

    scene.trace();
}
