use nalgebra_glm as glm;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_app_state(time: f32, canvas_width: f32, canvas_height: f32) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        time: time,
        canvas_width: canvas_width,
        canvas_height: canvas_height,
        ..*data.clone()
    })
}

pub fn get_current_app_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    })
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_height - y;
    let x_delta = x - data.mouse_x;
    let y_delta = inverted_y - data.mouse_y;
    let rotate_x_delta = if data.mouse_down { std::f32::consts::PI * x_delta / data.canvas_width } else { 0.0 };
    let rotate_y_delta = if data.mouse_down { std::f32::consts::PI * y_delta / data.canvas_height } else { 0.0 };
    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: inverted_y,
        rotation_x: data.rotation_x + rotate_x_delta,
        rotation_y: data.rotation_y - rotate_y_delta,
        ..*data.clone()
    })
}

pub struct AppState {
    pub time: f32,
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            time: 0.0,
            canvas_height: 0.0,
            canvas_width: 0.0,
            mouse_down: false,
            mouse_x: -1.0,
            mouse_y: -1.0,
            rotation_x: 2.8,
            rotation_y: 0.8,
        }
    }

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        let aspect_ratio = self.canvas_width / self.canvas_height;
        //glm::ortho(-aspect_ratio, aspect_ratio, -1.0, 1.0, -1.0, 1.0)
        let view = glm::look_at(
            &glm::vec3(0.0, 4.0, 0.0), // Camera is at (0,5,0), in World Space
            &glm::vec3(0.0, 0.0, 0.0), // and looks at the origin
            &glm::vec3(0.0, 0.0, 1.0), // Head is up
        );
        glm::perspective(aspect_ratio, 45.0 * std::f32::consts::PI / 180.0, 0.1, 100.0) * view
    }
}
