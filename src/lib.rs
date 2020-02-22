#[macro_use]
extern crate log;

mod drawing;
mod model;
mod state;
pub mod wasm;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub(crate) fn new(a: i32, b: i32) -> Self {
        if a < b {
            Range { start: a, end: b }
        } else {
            Range { start: b, end: a }
        }
    }
}

pub(crate) enum VirtualKey {
    Escape,
    Delete,
}

impl VirtualKey {
    pub(crate) fn from_str(key: &str) -> Option<Self> {
        let key = match key {
            "Delete" => VirtualKey::Delete,
            "Escape" => VirtualKey::Escape,
            _ => return None,
        };
        Some(key)
    }
}
