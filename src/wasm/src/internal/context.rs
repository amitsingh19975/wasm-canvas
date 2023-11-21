use std::cell::RefCell;

use super::{
    event::Event,
    geometry::{Circle, Line, Point, Rectangle, Text},
    node::Node,
    style::Style,
};
use gloo_utils::format::JsValueSerdeExt;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

pub(crate) const BOUNDING_BOX_MARGIN: f64 = 5.0;

#[wasm_bindgen]
pub struct Context {
    pub(crate) context: web_sys::CanvasRenderingContext2d,
    pub(crate) background_color: Option<String>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) selected_node: Option<usize>,
    pub(crate) hovered_node: Option<usize>,
    pub(crate) bounding_box_color: String,
    pub(crate) event: VecDeque<Event>,
    pub(crate) last_event: Option<Event>,
    pub(crate) drag_start_event: Option<Event>,
    is_dirty: bool,
}

impl Context {
    pub fn new<S: AsRef<str>>(id: S, background_color: Option<String>) -> Option<Self> {
        let document = web_sys::window()?.document()?;
        let canvas = document.get_element_by_id(id.as_ref())?;
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context_options = JsValue::from_serde(&serde_json::json!({
            "alpha": false,
            "desynchronized": true,
            "willReadFrequently": false,
        }))
        .ok()?;

        let context = canvas
            .get_context_with_context_options("2d", &context_options)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let nodes = Vec::new();

        let mut canvas = Context {
            context,
            background_color,
            nodes,
            selected_node: None,
            hovered_node: None,
            bounding_box_color: "#0ea5e9".to_string(),
            event: VecDeque::new(),
            last_event: None,
            drag_start_event: None,
            is_dirty: true,
        };
        let _ = canvas.clear_screen();

        Some(canvas)
    }

    fn push_unique_event(&mut self, event: Event) {
        if let Some(last_el) = self.event.back() {
            if last_el == &event {
                return;
            }
        }
        self.event.push_back(event);
        self.is_dirty = true;
    }

    pub fn on_mouse_down(&mut self, x: f64, y: f64, ts: f64) {
        self.push_unique_event(Event::from_mouse_down(x, y, ts));
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64, ts: f64) {
        self.push_unique_event(Event::from_mouse_move(x, y, ts));
    }

    pub fn on_mouse_up(&mut self, x: f64, y: f64, ts: f64) {
        self.push_unique_event(Event::from_mouse_up(x, y, ts));
    }

    pub fn on_key_down(&mut self, key: String, ts: f64) {
        self.push_unique_event(Event::from_key_down(key, ts));
    }

    pub fn on_key_up(&mut self, key: String, ts: f64) {
        self.push_unique_event(Event::from_key_up(key, ts));
    }

    pub fn on_key_press(&mut self, key: String, ts: f64) {
        self.push_unique_event(Event::from_key_press(key, ts));
    }

    pub fn shape(&self) -> (f64, f64) {
        let canvas = self.context.canvas();
        if let Some(canvas) = canvas {
            return (canvas.width() as f64, canvas.height() as f64);
        }

        (0.0, 0.0)
    }

    pub fn clear_screen(&mut self) -> Option<()> {
        let canvas = self.context.canvas()?;
        if let Some(color) = &self.background_color {
            self.context.set_fill_style(&JsValue::from(color));
            self.context
                .fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        } else {
            self.context
                .clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        }
        Some(())
    }

    pub fn render(&mut self) -> Option<()> {
        // if self.nodes.iter().all(|node| !node.is_dirty()) && !self.is_dirty {
        //     return Some(());
        // }

        self.clear_screen()?;
        let mut has_any_node_selected = false;
        let mut has_hovered_node = false;

        // super::log_utils::log(&format!("event: {:?}", self.event));

        loop {
            let event = self.event.pop_front();
            if event.is_none() {
                break;
            }

            let event = unsafe { event.unwrap_unchecked() };

            for (index, node) in self.nodes.iter().enumerate().rev() {
                if node.is_hovered(&event) {
                    if event.is_click(&mut self.last_event) {
                        self.selected_node = Some(index);
                        has_any_node_selected = true;
                    }
                    self.hovered_node = Some(index);
                    has_hovered_node = true;
                    break;
                }
            }

            if !has_hovered_node {
                self.hovered_node = None;
            }

            if !has_any_node_selected {
                if event.is_click(&mut self.last_event) {
                    self.selected_node = None;
                }
            }

            self.last_event = Some(event.clone());

            if event.mouse_down {
                self.drag_start_event = Some(event.clone());
                self.selected_node = self.hovered_node;
            } else if event.mouse_moved && self.drag_start_event.is_some() {
                // Do nothing
            } else {
                self.drag_start_event = None;
            }

            if let Some(drag_start_event) = &self.drag_start_event {
                self.selected_node = self.selected_node.or(self.hovered_node);
                if let Some(selected_node) = self.selected_node {
                    if event.mouse_moved {
                        if let Some(node) = self.nodes.get_mut(selected_node) {
                            let dx = event.mouse_x - drag_start_event.mouse_x;
                            let dy = event.mouse_y - drag_start_event.mouse_y;
                            node.translate(dx, dy);
                            self.drag_start_event = Some(event.clone());
                        }
                    }
                }
            }
        }

        for (index, node) in self.nodes.iter().enumerate() {
            let is_selected = Some(index) == self.selected_node;
            self.render_node(node, Some(index) == self.hovered_node, is_selected);
        }

        self.nodes.iter_mut().for_each(|node| {
            node.set_is_dirty(false);
        });

        self.is_dirty = true;

        Some(())
    }

    pub(crate) fn render_node(&self, node: &Node, has_bounding_box: bool, is_selected: bool) {
        match node {
            Node::Line(line, style, is_dirty) => {
                if *is_dirty || self.is_dirty {
                    line.render(&self, style, has_bounding_box, is_selected);
                }
            }
            Node::Rect(rect, style, is_dirty) => {
                if *is_dirty || self.is_dirty {
                    rect.render(&self, style, has_bounding_box, is_selected);
                }
            }
            Node::Circle(circle, style, is_dirty) => {
                if *is_dirty || self.is_dirty {
                    circle.render(&self, style, has_bounding_box, is_selected);
                }
            }
            Node::Text(text, style, is_dirty) => {
                if *is_dirty || self.is_dirty {
                    text.render(&self, style, has_bounding_box, is_selected);
                }
            }
        }
    }

    pub(crate) fn draw_bounding_box(
        &self,
        rect: Rectangle,
        color: &str,
        style: &Style,
        is_selected: bool,
    ) {
        self.context.save();
        if let Some((x, y)) = style.translate {
            let _ = self.context.translate(x, y);
        }
        if let Some((rotation, control_point)) = style.rotation {
            let (x, y) = control_point.resolve_coords(rect.left, rect.top, rect.width, rect.height);
            let _ = self.context.translate(x, y);
            let _ = self.context.rotate(rotation);
            let _ = self.context.translate(-x, -y);
        }

        self.context.begin_path();
        self.context.set_stroke_style(&JsValue::from(color));
        self.context
            .set_line_width(if is_selected { 2.5 } else { 1.0 });
        self.context.rect(
            rect.left - BOUNDING_BOX_MARGIN,
            rect.top - BOUNDING_BOX_MARGIN,
            rect.width + BOUNDING_BOX_MARGIN * 2.0,
            rect.height + BOUNDING_BOX_MARGIN * 2.0,
        );
        self.context.close_path();
        self.context.stroke();
        self.context.restore();
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_line(&mut self, start: Point, end: Point, style: Style) {
        let line = Line { start, end };
        let node = Node::Line(line, style, true);
        self.add_node(node);
    }

    pub fn add_rect(&mut self, top: f64, left: f64, width: f64, height: f64, style: Style) {
        let rect = Rectangle {
            top,
            left,
            width,
            height,
        };
        let node = Node::Rect(rect, style, true);
        self.add_node(node);
    }

    pub fn add_circle(&mut self, center: Point, radius: f64, style: Style) {
        let circle = Circle { center, radius };
        let node = Node::Circle(circle, style, true);
        self.add_node(node);
    }

    pub fn add_text(&mut self, text: String, position: Point, style: Style) {
        let text = Text {
            text,
            position,
            shape: RefCell::new(None),
        };
        let node = Node::Text(text, style, true);
        self.add_node(node);
    }
}
