use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // For dyn_into().

use crate::model::Model;
use crate::{Point2D, VirtualKey};

#[wasm_bindgen]
pub fn setup_for_debug() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap();
    info!("Debug initialized");
}

#[wasm_bindgen]
pub struct RangeCanvas {
    #[allow(unused)]
    canvas: web_sys::HtmlCanvasElement,

    model: Model,
}

#[wasm_bindgen]
impl RangeCanvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RangeCanvas, JsValue> {
        let document = web_sys::window()
            .expect("window")
            .document()
            .expect("document");

        let canvas = document
            .get_element_by_id("range-canvas")
            .expect("#range-canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        Ok(RangeCanvas {
            canvas,
            model: Model::new(),
        })
    }

    pub fn on_mouse_down(&mut self, x: i32, y: i32) {
        self.model.on_mouse_down(Point2D::new(x, y));
    }

    pub fn on_mouse_up(&mut self, x: i32, y: i32) {
        self.model.on_mouse_up(Point2D::new(x, y));
    }

    pub fn on_mouse_move(&mut self, x: i32, y: i32) {
        self.model.on_mouse_move(Point2D::new(x, y));
    }

    pub fn on_key_up(&mut self, key: &str) {
        if let Some(key) = VirtualKey::from_str(key) {
            self.model.on_key_up(key);
        }
    }
}
