use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;

use super::geometry::Rectangle;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum ControlPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Custom(f64, f64)
}

impl ControlPoint {
    pub(crate) fn from_str(s: &str) -> Self {
        match s {
            "tl" | "lt" => Self::TopLeft,
            "tr" | "rt" => Self::TopRight,
            "bl" | "lb" => Self::BottomLeft,
            "br" | "rb" => Self::BottomRight,
            "c" => Self::Center,
            _ => Self::Custom(0.0, 0.0),
        }
    }

    pub(crate) fn resolve_coords(&self, x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
        match *self {
            ControlPoint::TopLeft => (x, y),
            ControlPoint::TopRight => (x + width, y),
            ControlPoint::BottomLeft => (x, y + height),
            ControlPoint::BottomRight => (x + width, y + height),
            ControlPoint::Center => (x + width / 2.0, y + height / 2.0),
            ControlPoint::Custom(x, y) => (x, y)
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, Default)]
pub struct Style{
    pub fill_color: Option<String>,
    pub stroke_width: Option<f64>,
    pub stroke_color: Option<String>,
    pub font_size: Option<f64>,
    pub rotation: Option<(f64, ControlPoint)>,
    pub translate: Option<(f64, f64)>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn fill_color(&mut self, color: String) -> &mut Self {
        self.fill_color = Some(color);
        self
    }

    pub fn reset_fill_color(&mut self) -> &mut Self {
        self.fill_color = None;
        self
    }

    pub fn font_size(&mut self, size: f64) -> &mut Self {
        self.font_size = Some(size);
        self
    }

    pub fn reset_font_size(&mut self) -> &mut Self {
        self.font_size = None;
        self
    }

    pub fn stroke(&mut self, width: f64, color: String) -> &mut Self {
        self.stroke_width = Some(width);
        self.stroke_color = Some(color);
        self
    }

    pub fn reset_stroke(&mut self) -> &mut Self {
        self.stroke_width = None;
        self.stroke_color = None;
        self
    }

    pub fn translate(&mut self, x: f64, y: f64) -> &mut Self {
        self.translate = Some((x, y));
        self
    }

    pub fn reset_translate(&mut self) -> &mut Self {
        self.translate = None;
        self
    }

    pub fn rotation(&mut self, rotation: f64, control_point: Option<String>, x: Option<f64>, y: Option<f64>) -> &mut Self {
        let mut control_point = ControlPoint::from_str(&control_point.unwrap_or_default());
        if let ControlPoint::Custom(_, _) = control_point {
            let tx = x.unwrap_or_default();
            let ty = y.unwrap_or_default();
            control_point = ControlPoint::Custom(tx, ty);
        }
        self.rotation = Some((rotation, control_point));
        self
    }

    pub(crate) fn from_json(json: &JsValue) -> Self {
        return json.into_serde().unwrap_or_default();
    }

    pub(crate) fn apply_style(&self, ctx: &web_sys::CanvasRenderingContext2d, rect: Rectangle) {
        self.fill_color.as_ref().map(|color| ctx.set_fill_style(&JsValue::from(color)));
        self.stroke_width.as_ref().map(|width| ctx.set_line_width(*width));
        self.stroke_color.as_ref().map(|color| ctx.set_stroke_style(&JsValue::from(color)));
        self.font_size.as_ref().map(|size| ctx.set_font(&format!("{}px sans-serif", size)));
        self.translate.as_ref().map(|(x, y)| ctx.translate(*x, *y));
        self.rotation.as_ref().map(|(rotation, cp)| {
            let Rectangle{ top, left, width, height } = rect;
            let (ax, ay) = cp.resolve_coords(left, top, width, height);
            let _ = ctx.translate(ax, ay);
            let _ = ctx.rotate(*rotation);
            let _ = ctx.translate(-ax, -ay);
        });
    }

    pub(crate) fn patch(self, other: JsValue) -> Self {
        let mut res = Self::default();
        let other = Self::from_json(&other);
        res.fill_color = other.fill_color.or(self.fill_color);
        res.stroke_width = other.stroke_width.or(self.stroke_width);
        res.stroke_color = other.stroke_color.or(self.stroke_color);
        res.font_size = other.font_size.or(self.font_size);
        res.rotation = other.rotation.or(self.rotation);
        res.translate = other.translate.or(self.translate);
        res
    }
}