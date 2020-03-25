use kiss3d::camera::Camera;
use kiss3d::event::{Action, Key, MouseButton, WindowEvent};
use na::{self, Isometry3, Matrix4, Perspective3, Point3, Translation3, Vector2, Vector3};
use num::Zero;
use kiss3d::resource::ShaderUniform;
use std::f32;
use kiss3d::window::Canvas;

/// First-person camera mode.
///
///   * Left button press + drag - look around
///   * Right button press + drag - translates the camera position on the plane orthogonal to the
///   view direction
///   * Scroll in/out - zoom in/out
#[derive(Debug, Clone)]
pub struct PlayerCamera {
    eye: Point3<f32>,
    yaw: f32,
    pitch: f32,

    yaw_step: f32,
    pitch_step: f32,
    move_step: f32,
    rotate_button: Option<MouseButton>,
    drag_button: Option<MouseButton>,
    up_key: Option<Key>,
    down_key: Option<Key>,
    left_key: Option<Key>,
    right_key: Option<Key>,

    projection: Perspective3<f32>,
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
    proj_view: Matrix4<f32>,
    inverse_proj_view: Matrix4<f32>,
    last_cursor_pos: Vector2<f32>,
    coord_system: CoordSystem,
}

impl PlayerCamera {
    /// Creates a first person camera with default sensitivity values.
    pub fn new(eye: Point3<f32>, at: Point3<f32>) -> PlayerCamera {
        PlayerCamera::new_with_frustrum(f32::consts::PI / 4.0, 0.1, 1024.0, eye, at)
    }

    /// Creates a new first person camera with default sensitivity values.
    pub fn new_with_frustrum(
        fov: f32,
        znear: f32,
        zfar: f32,
        eye: Point3<f32>,
        at: Point3<f32>,
    ) -> PlayerCamera {
        let mut res = PlayerCamera {
            eye: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            yaw_step: 0.005,
            pitch_step: 0.005,
            move_step: 0.5,
            rotate_button: Some(MouseButton::Button1),
            drag_button: Some(MouseButton::Button2),
            up_key: Some(Key::Up),
            down_key: Some(Key::Down),
            left_key: Some(Key::Left),
            right_key: Some(Key::Right),
            projection: Perspective3::new(800.0 / 600.0, fov, znear, zfar),
            proj: na::zero(),
            view: na::zero(),
            proj_view: na::zero(),
            inverse_proj_view: na::zero(),
            last_cursor_pos: na::zero(),
            coord_system: CoordSystem::RightHandYUp,
        };

        res.look_at(eye, at);

        res
    }

    /// Sets the translational increment per arrow press.
    ///
    /// The default value is 0.5.
    #[inline]
    pub fn set_move_step(&mut self, step: f32) {
        self.move_step = step;
    }

    /// Sets the pitch increment per mouse movement.
    ///
    /// The default value is 0.005.
    #[inline]
    pub fn set_pitch_step(&mut self, step: f32) {
        self.pitch_step = step;
    }

    /// Sets the yaw increment per mouse movement.
    ///
    /// The default value is 0.005.
    #[inline]
    pub fn set_yaw_step(&mut self, step: f32) {
        self.yaw_step = step;
    }

    /// Gets the translational increment per arrow press.
    #[inline]
    pub fn move_step(&self) -> f32 {
        self.move_step
    }

    /// Gets the pitch increment per mouse movement.
    #[inline]
    pub fn pitch_step(&self) -> f32 {
        self.pitch_step
    }

    /// Gets the yaw  increment per mouse movement.
    #[inline]
    pub fn yaw_step(&self) -> f32 {
        self.yaw_step
    }

    /// Changes the orientation and position of the camera to look at the specified point.
    pub fn look_at(&mut self, eye: Point3<f32>, at: Point3<f32>) {
        let dist = (eye - at).norm();

        let pitch;
        let yaw;
        match self.coord_system {
            CoordSystem::RightHandYUp => {
                pitch = ((at.y - eye.y) / dist).acos();
                yaw = (at.z - eye.z).atan2(at.x - eye.x);
            }
            CoordSystem::RightHandZUp => {
                pitch = ((at.z - eye.z) / dist).acos();
                yaw = (eye.y - at.y).atan2(at.x - eye.x);
            }
        }

        self.eye = eye;
        self.yaw = yaw;
        self.pitch = pitch;
        self.update_projviews();
    }

    /// The point the camera is looking at.
    pub fn at(&self) -> Point3<f32> {
        let ax = self.eye.x + self.yaw.cos() * self.pitch.sin();
        let ay;
        let az;
        match self.coord_system {
            CoordSystem::RightHandYUp => {
                ay = self.eye.y + self.pitch.cos();
                az = self.eye.z + self.yaw.sin() * self.pitch.sin();
            }
            CoordSystem::RightHandZUp => {
                ay = self.eye.y - self.yaw.sin() * self.pitch.sin();
                az = self.eye.z + self.pitch.cos();
            }
        }

        Point3::new(ax, ay, az)
    }

    fn update_restrictions(&mut self) {
        if self.pitch <= 0.01 {
            self.pitch = 0.01
        }

        let _pi: f32 = f32::consts::PI;
        if self.pitch > _pi - 0.01 {
            self.pitch = _pi - 0.01
        }
    }

    /// The button used to rotate the PlayerCamera camera.
    pub fn rotate_button(&self) -> Option<MouseButton> {
        self.rotate_button
    }

    /// Set the button used to rotate the PlayerCamera camera.
    /// Use None to disable rotation.
    pub fn rebind_rotate_button(&mut self, new_button: Option<MouseButton>) {
        self.rotate_button = new_button;
    }

    /// The button used to drag the PlayerCamera camera.
    pub fn drag_button(&self) -> Option<MouseButton> {
        self.drag_button
    }

    /// Set the button used to drag the PlayerCamera camera.
    /// Use None to disable dragging.
    pub fn rebind_drag_button(&mut self, new_button: Option<MouseButton>) {
        self.drag_button = new_button;
    }

    /// The movement button for up.
    pub fn up_key(&self) -> Option<Key> {
        self.up_key
    }

    /// The movement button for down.
    pub fn down_key(&self) -> Option<Key> {
        self.down_key
    }

    /// The movement button for left.
    pub fn left_key(&self) -> Option<Key> {
        self.left_key
    }

    /// The movement button for right.
    pub fn right_key(&self) -> Option<Key> {
        self.right_key
    }

    /// Set the movement button for up.
    /// Use None to disable movement in this direction.
    pub fn rebind_up_key(&mut self, new_key: Option<Key>) {
        self.up_key = new_key;
    }

    /// Set the movement button for down.
    /// Use None to disable movement in this direction.
    pub fn rebind_down_key(&mut self, new_key: Option<Key>) {
        self.down_key = new_key;
    }

    /// Set the movement button for left.
    /// Use None to disable movement in this direction.
    pub fn rebind_left_key(&mut self, new_key: Option<Key>) {
        self.left_key = new_key;
    }

    /// Set the movement button for right.
    /// Use None to disable movement in this direction.
    pub fn rebind_right_key(&mut self, new_key: Option<Key>) {
        self.right_key = new_key;
    }

    /// Disable the movement buttons for up, down, left and right.
    pub fn unbind_movement_keys(&mut self) {
        self.up_key = None;
        self.down_key = None;
        self.left_key = None;
        self.right_key = None;
    }

    #[doc(hidden)]
    pub fn handle_left_button_displacement(&mut self, dpos: &Vector2<f32>) {
        self.yaw = self.yaw + dpos.x * self.yaw_step;
        self.pitch = self.pitch + dpos.y * self.pitch_step;

        self.update_restrictions();
        self.update_projviews();
    }

    #[doc(hidden)]
    pub fn handle_right_button_displacement(&mut self, dpos: &Vector2<f32>) {
        let at = self.at();
        let dir = (at - self.eye).normalize();
        let tangent = self.coord_system.up_axis().cross(&dir).normalize();
        let bitangent = dir.cross(&tangent);

        self.eye = self.eye + tangent * (0.01 * dpos.x / 10.0) + bitangent * (0.01 * dpos.y / 10.0);
        self.update_restrictions();
        self.update_projviews();
    }

    #[doc(hidden)]
    pub fn handle_scroll(&mut self, yoff: f32) {
        let front = self.observer_frame() * Vector3::z();

        self.eye = self.eye + front * (self.move_step * yoff);

        self.update_restrictions();
        self.update_projviews();
    }

    fn update_projviews(&mut self) {
        self.view = self.view_transform().to_homogeneous();
        self.proj = *self.projection.as_matrix();
        self.proj_view = self.proj * self.view;
        let _ = self
            .proj_view
            .try_inverse()
            .map(|inverse_proj| self.inverse_proj_view = inverse_proj);
    }

    /// The direction this camera is looking at.
    pub fn eye_dir(&self) -> Vector3<f32> {
        (self.at() - self.eye).normalize()
    }

    /// The direction this camera is being moved by the keyboard keys for a given set of key states.
    pub fn move_dir(&self, up: bool, down: bool, right: bool, left: bool) -> Vector3<f32> {
        let t = self.observer_frame();
        let frontv = t * Vector3::z();
        let rightv = t * Vector3::x();

        let mut movement = na::zero::<Vector3<f32>>();

        if up {
            movement = movement + frontv
        }

        if down {
            movement = movement - frontv
        }

        if right {
            movement = movement - rightv
        }

        if left {
            movement = movement + rightv
        }

        if movement.is_zero() {
            movement
        } else {
            movement.normalize()
        }
    }

    /// Translates in-place this camera by `t`.
    #[inline]
    pub fn translate_mut(&mut self, t: &Translation3<f32>) {
        let new_eye = t * self.eye;

        self.set_eye(new_eye);
    }

    /// Translates this camera by `t`.
    #[inline]
    pub fn translate(&self, t: &Translation3<f32>) -> PlayerCamera {
        let mut res = self.clone();
        res.translate_mut(t);
        res
    }

    /// Sets the eye of this camera to `eye`.
    #[inline]
    fn set_eye(&mut self, eye: Point3<f32>) {
        self.eye = eye;
        self.update_restrictions();
        self.update_projviews();
    }

    /// Sets the Up vector of this camera.
    #[inline]
    pub fn set_up_axis(&mut self, up_axis: Vector3<f32>) {
        let new_coord_system;
        if up_axis == Vector3::y() {
            new_coord_system = CoordSystem::RightHandYUp;
        } else if up_axis == Vector3::z() {
            new_coord_system = CoordSystem::RightHandZUp;
        } else {
            panic!("This up_axis is not supported: {:?}", up_axis);
        }
        if self.coord_system != new_coord_system {
            // Since setting the up axis changes the meaning of pitch and yaw
            // angles, we need to recalculate them in order to preserve the eye
            // position.
            let old_at = self.at();
            self.coord_system = new_coord_system;
            self.look_at(self.eye, old_at);
        }
    }

    /// The camera observer local frame.
    fn observer_frame(&self) -> Isometry3<f32> {
        Isometry3::face_towards(&self.eye, &self.at(), &self.coord_system.up_axis())
    }
}

impl Camera for PlayerCamera {
    fn clip_planes(&self) -> (f32, f32) {
        (self.projection.znear(), self.projection.zfar())
    }

    /// The camera view transformation (i-e transformation without projection).
    fn view_transform(&self) -> Isometry3<f32> {
        Isometry3::look_at_rh(&self.eye, &self.at(), &self.coord_system.up_axis())
    }

    fn handle_event(&mut self, canvas: &Canvas, event: &WindowEvent) {
        match *event {
            WindowEvent::CursorPos(x, y, _) => {
                let curr_pos = Vector2::new(x as f32, y as f32);

                if let Some(rotate_button) = self.rotate_button {
                    if canvas.get_mouse_button(rotate_button) == Action::Press {
                        let dpos = curr_pos - self.last_cursor_pos;
                        self.handle_left_button_displacement(&dpos)
                    }
                }

                if let Some(drag_button) = self.drag_button {
                    if canvas.get_mouse_button(drag_button) == Action::Press {
                        let dpos = curr_pos - self.last_cursor_pos;
                        self.handle_right_button_displacement(&dpos)
                    }
                }

                self.last_cursor_pos = curr_pos;
            }
            WindowEvent::Scroll(_, off, _) => self.handle_scroll(off as f32),
            WindowEvent::FramebufferSize(w, h) => {
                self.projection.set_aspect(w as f32 / h as f32);
                self.update_projviews();
            }
            _ => {}
        }
    }

    fn eye(&self) -> Point3<f32> {
        self.eye
    }

    fn transformation(&self) -> Matrix4<f32> {
        self.proj_view
    }

    fn inverse_transformation(&self) -> Matrix4<f32> {
        self.inverse_proj_view
    }

    #[inline]
    fn upload(
        &self,
        _: usize,
        proj: &mut ShaderUniform<Matrix4<f32>>,
        view: &mut ShaderUniform<Matrix4<f32>>,
    ) {
        proj.upload(&self.proj);
        view.upload(&self.view);
    }

    fn update(&mut self, canvas: &Canvas) {
        let up = check_optional_key_state(canvas, self.up_key, Action::Press);
        let down = check_optional_key_state(canvas, self.down_key, Action::Press);
        let right = check_optional_key_state(canvas, self.right_key, Action::Press);
        let left = check_optional_key_state(canvas, self.left_key, Action::Press);
        let dir = self.move_dir(up, down, right, left);

        let move_amount = dir * self.move_step;
        self.translate_mut(&Translation3::from(move_amount));
    }
}

fn check_optional_key_state(canvas: &Canvas, key: Option<Key>, key_state: Action) -> bool {
    if let Some(actual_key) = key {
        canvas.get_key(actual_key) == key_state
    } else {
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CoordSystem {
    RightHandYUp,
    RightHandZUp,
}

impl CoordSystem {
    #[inline]
    fn up_axis(self) -> Vector3<f32> {
        match self {
            CoordSystem::RightHandYUp => Vector3::y(),
            CoordSystem::RightHandZUp => Vector3::z(),
        }
    }
}