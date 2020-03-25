extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;

mod core;

use na::{
    Point3,
    Vector3, 
    UnitQuaternion
};

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::window::State;
use kiss3d::scene::SceneNode;

fn main() {
    let mut window = Window::new("kiss");

    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut cam = core::player_camera::PlayerCamera::new(eye, at);
    window.render_with_camera(&mut cam);

    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(0.0, 0.45, 0.30);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let app = AppState{ root: cube, rot: rot };

    window.render_loop(app);
}

struct AppState {
    root: SceneNode,
    rotation: UnitQuaternion<f32>
}

impl State for AppState {
    
    fn step(&mut self, _:&mut Window) {
    }

}

