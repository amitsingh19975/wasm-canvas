use std::cell::RefCell;

use super::{context::{Context, BOUNDING_BOX_MARGIN}, style::Style};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point { x, y }
    }
}

impl Point {
    fn rotate(&self, angle: f64) -> Self {
        let (x, y) = (
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
        );
        Self { x, y }
    }

    fn translate(&self, tx: f64, ty: f64) -> Self {
        Self {x: self.x + tx, y: self.y + ty }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub text: String,
    pub position: Point,
    pub shape: RefCell<Option<Size>>,
}

impl Line {
    pub fn is_hovered(&self, x: f64, y: f64, style: &Style) -> bool {
        let (tx, ty) = style.translate.unwrap_or_default();
        let (sx, sy) = (self.start.x + tx, self.start.y + ty);
        let (ex, ey) = (self.end.x + tx, self.end.y + ty);

        let dx = ex - sx;
        let dy = ey - sy;
        let d = dx * dx + dy * dy; // dot product
        let a = ((x - sx) * dx + (y - sy) * dy) / d; // normalized dot product (projected point cosine)

        if a.is_nan() {
            return false;
        }

        let px = sx + a * dx; // projected point
        let py = sy + a * dy; // projected point
        let s = ((sy - ey) * x + (ex - sx) * y + (ey * sx - ex * sy))
            / ((sy - ey) * sy + (ex - sx) * ex); // distance from line cross product (projected point sine)
        let distance = ((x - px) * (x - px) + (y - py) * (y - py)).sqrt();
        if distance <= (style.stroke_width.unwrap_or_default() / 2.0)
            && a >= 0.0
            && a <= 1.0
            && s >= 0.0
            && s <= 1.0
        {
            return true;
        }
        false
    }

    pub(crate) fn render(&self, ctx: &Context, style: &Style, has_bounding_box: bool, is_selected: bool) {
        ctx.context.save();
        ctx.context.begin_path();
        let rect = Rectangle {
            top: self.start.y,
            left: self.start.x,
            width: self.end.x - self.start.x,
            height: self.end.y - self.start.y,
        };
        style.apply_style(&ctx.context, rect);
        ctx.context.move_to(self.start.x, self.start.y);
        ctx.context.line_to(self.end.x, self.end.y);
        if style.stroke_width.is_some() {
            ctx.context.stroke();
        }
        ctx.context.restore();

        if has_bounding_box || is_selected {
            ctx.draw_bounding_box(rect, &ctx.bounding_box_color, &style, is_selected);
        }
    }

    pub(crate) fn translate(&mut self, x: f64, y: f64) {
        self.start.x += x;
        self.start.y += y;
        self.end.x += x;
        self.end.y += y;
    }
}

impl Rectangle {
    fn get_rect(&self) -> Rectangle {
        self.clone()
    }

    pub fn is_hovered(&self, x: f64, y: f64, style: &Style) -> bool {

        let (tx, ty) = style.translate.unwrap_or_default();
        let width = style.stroke_width.unwrap_or_default() / 2.0;
        let left = self.left - width;
        let top = self.top - width;
        let right = left + self.width + width * 2.0;
        let bottom = top + self.height + width * 2.0;

        let mut tl = Point { x: left, y: top };
        let mut tr = Point { x: right, y: top };
        let mut bl = Point { x: left, y: bottom };
        let mut br = Point {
            x: right,
            y: bottom,
        };

        let zero = Point { x: 0.0, y: 0.0 };

        if let Some((angle, cp)) = style.rotation {
            let (ox, oy) = cp.resolve_coords(left, top, right - left, bottom - top);
            tl = zero.translate(-ox, -oy).translate(tl.x, tl.y).rotate(angle).translate(ox, oy).translate(tx, ty);
            tr = zero.translate(-ox, -oy).translate(tr.x, tr.y).rotate(angle).translate(ox, oy).translate(tx, ty);
            bl = zero.translate(-ox, -oy).translate(bl.x, bl.y).rotate(angle).translate(ox, oy).translate(tx, ty);
            br = zero.translate(-ox, -oy).translate(br.x, br.y).rotate(angle).translate(ox, oy).translate(tx, ty);
        } else {
            tl = tl.translate(tx, ty);
            tr = tr.translate(tx, ty);
            bl = bl.translate(tx, ty);
            br = br.translate(tx, ty);
        }

        // ctx.context.begin_path();
        // ctx.context.set_stroke_style(&JsValue::from("red"));
        // ctx.context.move_to(tl.x, tl.y);
        // ctx.context.line_to(tr.x, tr.y);
        // ctx.context.line_to(br.x, br.y);
        // ctx.context.line_to(bl.x, bl.y);
        // ctx.context.line_to(tl.x, tl.y);
        // ctx.context.close_path();
        // ctx.context.stroke();

        // log_utils::log(&format!("{}, {} | tl: {:?} tr: {:?} bl: {:?} br: {:?}",x, y, tl, tr, bl, br));

        // Ray casting algorithm
        {

            let mut counter = 0;
            let ps = [tl, tr, br, bl];

            for (p1, p2) in ps.iter().zip(ps.iter().cycle().skip(1)) {
                let Point { x: x1, y: y1 } = *p1;
                let Point { x: x2, y: y2 } = *p2;
                
                // log_utils::log(&format!("counter: {} | p1: {:?}, p2: {:?}", counter, p1, p2));

                if (y < y1) != (y < y2) {
                    let ratio = (y - y1) / (y2 - y1);
                    let max_x = (x2 - x1) * ratio + x1;
                    if x < max_x {
                        counter += 1;
                    }
                }
            }

            if (counter & 1) != 0 {
                return true;
            }
        }

        
        false
    }

    pub(crate) fn render(&self, ctx: &Context, style: &Style, has_bounding_box: bool, is_selected: bool) {
        ctx.context.save();
        ctx.context.begin_path();
        let rect = self.get_rect();

        style.apply_style(&ctx.context, rect);
        ctx.context
            .rect(self.left, self.top, self.width, self.height);
        if style.stroke_width.is_some() {
            ctx.context.stroke();
        }
        if style.fill_color.is_some() {
            ctx.context.fill();
        }
        ctx.context.restore();

        if has_bounding_box || is_selected {
            ctx.draw_bounding_box(rect, &ctx.bounding_box_color, &style, is_selected);
        }
    }

    pub(crate) fn translate(&mut self, x: f64, y: f64) {
        self.left += x;
        self.top += y;
    }

}

impl Circle {
    pub fn is_hovered(&self, x: f64, y: f64, style: &Style) -> bool {
        let (tx, ty) = style.translate.unwrap_or_default();

        let dx = x - self.center.x - tx;
        let dy = y - self.center.y - ty;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance <= self.radius + style.stroke_width.unwrap_or_default() / 2.0 {
            return true;
        }
        false
    }

    pub(crate) fn render(&self, ctx: &Context, style: &Style, has_bounding_box: bool, is_selected: bool) {
        ctx.context.save();
        ctx.context.begin_path();

        let rect = Rectangle {
            top: self.center.y - self.radius,
            left: self.center.x - self.radius,
            width: self.radius * 2.0,
            height: self.radius * 2.0,
        };

        style.apply_style(&ctx.context, rect);

        ctx.context
            .arc(
                self.center.x,
                self.center.y,
                self.radius,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
        if style.stroke_width.is_some() {
            ctx.context.stroke();
        }
        if style.fill_color.is_some() {
            ctx.context.fill();
        }
        ctx.context.restore();

        if has_bounding_box || is_selected {
            ctx.draw_bounding_box(rect, &ctx.bounding_box_color, &style, is_selected);
        }
    }

    pub(crate) fn translate(&mut self, x: f64, y: f64) {
        self.center.x += x;
        self.center.y += y;
    }
}

impl Text {

    fn get_rect(&self) -> Rectangle {
        let (width, height) = self
            .shape
            .borrow()
            .and_then(|size| Some((size.width, size.height)))
            .unwrap_or((0.0, 0.0));
        let top = self.position.y;
        let left = self.position.x;

        Rectangle {
            top,
            left,
            width,
            height,
        }
    }

    fn get_wire_frame_rect(&self) -> Rectangle {
        let rect = self.get_rect();
        Rectangle {
            left: rect.left - BOUNDING_BOX_MARGIN,
            top: rect.top - BOUNDING_BOX_MARGIN,
            width: rect.width + BOUNDING_BOX_MARGIN * 2.0,
            height: rect.height + BOUNDING_BOX_MARGIN * 2.0,
        }
    }

    pub fn is_hovered(&self, x: f64, y: f64, style: &Style) -> bool {
        return self.get_wire_frame_rect().is_hovered(x, y, style);
    }

    pub(crate) fn render(&self, ctx: &Context, style: &Style, has_bounding_box: bool, is_selected: bool) {
        let mut rect = Rectangle {
            top: self.position.y,
            left: self.position.x,
            width: 0.0,
            height: 0.0,
        };

        {
            ctx.context.save();
            style.apply_style(&ctx.context, rect);

            let measured_text = ctx.context.measure_text(&self.text).unwrap();
            let width = measured_text.width();
            let height = style.font_size.unwrap_or_default();
            rect.width = width;
            rect.height = height;
            *self.shape.borrow_mut() = Some(Size { width, height });
            ctx.context.restore();
        }

        // grid
        // {
        //     let (w, h) = ctx.shape();
        //     let grid_width = 100.0;
        //     let grid_iter = (w.max(h) / grid_width).floor() as usize;
        //     let grid_color = "rgba(255, 255, 255, 0.1)";

        //     ctx.context.save();
        //     for i  in 0..=grid_iter {
        //         ctx.context.begin_path();
        //         ctx.context.set_stroke_style(&JsValue::from(grid_color));

        //         ctx.context.set_line_width(4.0);
        //         let start = i as f64 * grid_width;
        //         ctx.context.move_to(start, 0.0);
        //         ctx.context.line_to(start, h);

        //         ctx.context.move_to(0.0, start);
        //         ctx.context.line_to(w, start);

        //         ctx.context.set_font("20px Arial");
        //         ctx.context.set_fill_style(&JsValue::from("white"));
        //         ctx.context.fill_text(&format!("{}", i as f64 * grid_width), start, 20.0).unwrap();
                
        //         ctx.context.fill_text(&format!("{}", i as f64 * grid_width), 0.0, i as f64 * grid_width).unwrap();
                
        //         ctx.context.stroke();
        //     }
        //     ctx.context.restore();

        // }


        ctx.context.save();
        style.apply_style(&ctx.context, rect);

        ctx.context.set_text_baseline("bottom");

        if style.fill_color.is_some() {
            ctx.context
                .fill_text(&self.text, self.position.x, self.position.y + rect.height)
                .unwrap();
        } else if style.stroke_width.is_some() {
            ctx.context
                .stroke_text(&self.text, self.position.x, self.position.y + rect.height)
                .unwrap();
        }
        ctx.context.restore();

        if has_bounding_box || is_selected {
            ctx.draw_bounding_box(rect, &ctx.bounding_box_color, &style, is_selected);
        }

    }

    pub(crate) fn translate(&mut self, x: f64, y: f64) {
        self.position.x += x;
        self.position.y += y;
    }
}
