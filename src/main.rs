use rayrs::scene::Scene;

fn main() {
    let scene = Scene::new("Examples/simple.json");

    scene.trace();
}
