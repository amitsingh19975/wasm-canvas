#[derive(Debug, Clone, Default, PartialOrd)]
pub(crate) struct Event {
    pub(crate) mouse_x: f64,
    pub(crate) mouse_y: f64,
    pub(crate) key: Option<String>,
    pub(crate) key_pressed: bool,
    pub(crate) key_up: bool,
    pub(crate) key_down: bool,
    pub(crate) mouse_up: bool,
    pub(crate) mouse_down: bool,
    pub(crate) mouse_moved: bool,
    pub(crate) timestamp: u128,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        (self.timestamp == other.timestamp)
            && (self.key_down == other.key_down)
            && (self.key_up == other.key_up)
            && (self.key_pressed == other.key_pressed)
            && (self.mouse_down == other.mouse_down)
            && (self.mouse_up == other.mouse_up)
            && (self.mouse_moved == other.mouse_moved)
            && (self.key == other.key)
    }
}

impl Eq for Event {}

impl Event {
    pub(crate) fn from_mouse_up(x: f64, y: f64, ts: f64) -> Self {
        Self {
            mouse_x: x,
            mouse_y: y,
            mouse_up: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn from_mouse_down(x: f64, y: f64, ts: f64) -> Self {
        Self {
            mouse_x: x,
            mouse_y: y,
            mouse_down: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn from_mouse_move(x: f64, y: f64, ts: f64) -> Self {
        Self {
            mouse_x: x,
            mouse_y: y,
            mouse_moved: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn from_key_down(key: String, ts: f64) -> Self {
        Self {
            key: Some(key),
            key_down: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn from_key_up(key: String, ts: f64) -> Self {
        Self {
            key: Some(key),
            key_up: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn from_key_press(key: String, ts: f64) -> Self {
        Self {
            key: Some(key),
            key_pressed: true,
            timestamp: ts as u128,
            ..Default::default()
        }
    }

    pub(crate) fn is_click(&self, last_event_opt: &mut Option<Event>) -> bool {
        if let Some(last_event) = last_event_opt {
            let diff = self.timestamp - last_event.timestamp;
            if diff > 500 || !last_event.mouse_down {
                last_event_opt.take();
                return false;
            }

            if self.mouse_up {
                last_event_opt.take();
                return true;
            }
        }
        false
    }
}
