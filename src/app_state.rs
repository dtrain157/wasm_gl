use nalgebra_glm as glm;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_app_state(canvas_width: f32, canvas_height: f32) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_width: canvas_width,
        canvas_height: canvas_height,
        ..*data.clone()
    })
}

pub fn get_current_app_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.0,
            canvas_width: 0.0,
        }
    }

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        let aspect_ratio = self.canvas_width / self.canvas_height;
        //glm::ortho(-aspect_ratio, aspect_ratio, -1.0, 1.0, -1.0, 1.0)
        let view = glm::look_at(
            &glm::vec3(0.0, 0.0, 1.0), // Camera is at (0,0,1), in World Space
            &glm::vec3(0.0, 0.0, 0.0), // and looks at the origin
            &glm::vec3(0.0, 1.0, 0.0), // Head is up
        );

        glm::perspective(aspect_ratio, 45.0 * std::f32::consts::PI / 180.0, 0.1, 100.0) * view
    }
}
