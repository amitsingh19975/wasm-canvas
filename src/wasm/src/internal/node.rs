use super::{geometry::{Line, Rectangle, Circle, Text}, style::Style, context::Context, event::Event};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Node {
    Line(Line, Style, bool),
    Rect(Rectangle, Style, bool),
    Circle(Circle, Style, bool),
    Text(Text, Style, bool),
}

impl Node {
    pub(crate) fn get_style_mut(&mut self) -> &mut Style {
        match self {
            Node::Line(_, style, ..) => style,
            Node::Rect(_, style, ..) => style,
            Node::Circle(_, style, ..) => style,
            Node::Text(_, style, ..) => style,
        }
    }

    pub(crate) fn set_style(&mut self, style: JsValue) {
        let temp = self.clone();
        *self = match temp {
            Node::Line(v, os, ..) => Node::Line(v, os.patch(style), true),
            Node::Rect(v, os, ..) => Node::Rect(v, os.patch(style), true),
            Node::Circle(v, os, ..) => Node::Circle(v, os.patch(style), true),
            Node::Text(v, os, ..) =>  Node::Text(v, os.patch(style), true)
        }
    }

    pub(crate) fn is_hovered(&self, event: &Event) -> bool {
        if !(event.mouse_moved || event.mouse_down || event.mouse_up) {
            return false;
        }

        let (x, y) = (event.mouse_x, event.mouse_y);

        match self {
            Node::Line(line, style, ..) => line.is_hovered(x, y, style),
            Node::Rect(rect, style, ..) => rect.is_hovered(x, y, style),
            Node::Circle(circle, style, ..) => circle.is_hovered(x, y, style),
            Node::Text(text, style, ..) => text.is_hovered(x, y, style),
        }
    }

    pub(crate) fn translate(&mut self, x: f64, y: f64) {
        match self {
            Node::Line(line, ..) => line.translate(x, y),
            Node::Rect(rect, ..) => rect.translate(x, y),
            Node::Circle(circle, ..) => circle.translate(x, y),
            Node::Text(text, ..) => text.translate(x, y),
        }

        self.set_is_dirty(true);
    }

    pub(crate) fn set_is_dirty(&mut self, is_dirty: bool) {
        match self {
            Node::Line(_, _, is_dirty_ref) => *is_dirty_ref = is_dirty,
            Node::Rect(_, _, is_dirty_ref) => *is_dirty_ref = is_dirty,
            Node::Circle(_, _, is_dirty_ref) => *is_dirty_ref = is_dirty,
            Node::Text(_, _, is_dirty_ref) => *is_dirty_ref = is_dirty,
        }
    }

    pub(crate) fn is_dirty(&self) -> bool {
        match self {
            Node::Line(_, _, is_dirty_ref) => *is_dirty_ref,
            Node::Rect(_, _, is_dirty_ref) => *is_dirty_ref,
            Node::Circle(_, _, is_dirty_ref) => *is_dirty_ref,
            Node::Text(_, _, is_dirty_ref) => *is_dirty_ref,
        }
    }
}

#[wasm_bindgen]
pub struct NodeRef {
    ptr: *mut Context,
    id: isize,
}

impl NodeRef {

    pub fn new(context: &Context, id: usize) -> Self {
        Self {
            ptr: context as *const Context as *mut Context,
            id: id as isize
        }
    }

    fn get_context(&self) -> &Context {
        unsafe { &*self.ptr }
    }
    
    fn get_context_mut(&mut self) -> &mut Context {
        unsafe { &mut *self.ptr }
    }
}

#[wasm_bindgen]
impl NodeRef {
    pub fn translate(&mut self, x: f64, y: f64) {
        if self.id < 0 {
            return;
        }

        let id = self.id as usize;
        if let Some(node) = self.get_context_mut().nodes.get_mut(id) {
            node.get_style_mut().translate(x, y);
            node.set_is_dirty(true);
        }
    }

    pub fn set_style(&mut self, style: JsValue) {
        if self.id < 0 {
            return;
        }

        let id = self.id as usize;
        if let Some(node) = self.get_context_mut().nodes.get_mut(id) {
            node.set_style(style);
        }
    }

    pub fn rotation(&mut self, angle: f64, control_point: Option<String>, x: Option<f64>, y: Option<f64>) {
        if self.id < 0 {
            return;
        }

        let id = self.id as usize;
        if let Some(node) = self.get_context_mut().nodes.get_mut(id) {
            node.get_style_mut().rotation(angle, control_point, x, y);
            node.set_is_dirty(true);
        }
    }

    pub fn remove(mut self) {
        if self.id < 0 {
            return;
        }

        let id = self.id as usize;
        self.get_context_mut().nodes.remove(id);
    }

    pub fn is_hovered(&self, x: f64, y: f64) -> bool {
        if self.id < 0 {
            return false;
        }

        let id = self.id as usize;
        if let Some(node) = self.get_context().nodes.get(id) {
            return node.is_hovered(&Event::from_mouse_move(x, y, 0.0));
        }

        false
    }
}
