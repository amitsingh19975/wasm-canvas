mod utils;
mod internal;

use internal::{context::Context, style::Style};
use wasm_bindgen::prelude::*;

use crate::{internal::node::NodeRef, utils::set_panic_hook};


#[wasm_bindgen]
pub struct Canvas {
    inner: Context
}

impl Canvas {
    fn make_node_ref(&self) -> NodeRef {
        let id = self.inner.nodes.len() - 1;
        NodeRef::new(&self.inner, id)
    }
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, background_color: Option<String>) -> Canvas {
        set_panic_hook();
        
        let inner = Context::new(id, background_color).unwrap();
        
        Self {
            inner
        }
    }

    pub fn render(&mut self) {
        let _ = self.inner.render();
    }

    pub fn get_js_canvas(&self) -> Option<web_sys::HtmlCanvasElement> {
        self.inner.context.canvas()
    }

    pub fn add_line(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, style: JsValue) -> NodeRef {
        self.inner.add_line(
            (start_x, start_y).into(), 
            (end_x, end_y).into(),
            Style::from_json(&style)
        );
        self.make_node_ref()
    }

    pub fn add_rect(&mut self, top: f64, left: f64, width: f64, height: f64, style: JsValue) -> NodeRef {
        self.inner.add_rect(
            top, 
            left, 
            width, 
            height,
            Style::from_json(&style)
        );
        self.make_node_ref()
    }

    pub fn add_circle(&mut self, center_x: f64, center_y: f64, radius: f64, style: JsValue) -> NodeRef {
        self.inner.add_circle(
            (center_x, center_y).into(), 
            radius,
            Style::from_json(&style)
        );
        self.make_node_ref()
    }

    pub fn add_text(&mut self, text: String, x: f64, y: f64, style: JsValue) -> NodeRef {
        self.inner.add_text(
            text,
            (x, y).into(),
            Style::from_json(&style)
        );
        self.make_node_ref()
    }

    pub fn on_mouse_down(&mut self, x: f64, y: f64, ts: f64) {
        self.inner.on_mouse_down(x, y, ts);
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64, ts: f64) {
        self.inner.on_mouse_move(x, y, ts);
    }

    pub fn on_mouse_up(&mut self, x: f64, y: f64, ts: f64) {
        self.inner.on_mouse_up(x, y, ts);
    }

    pub fn on_key_down(&mut self, key: String, ts: f64) {
        self.inner.on_key_down(key, ts);
    }

    pub fn on_key_up(&mut self, key: String, ts: f64) {
        self.inner.on_key_up(key, ts);
    }

    pub fn on_key_press(&mut self, key: String, ts: f64) {
        self.inner.on_key_press(key, ts);
    }
}
