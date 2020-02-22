use wasm_bindgen::prelude::*;

use crate::Range;

#[wasm_bindgen(raw_module = "../drawing.js")]
extern "C" {
    fn invalidate();
    fn draw_selected_range(start_x: i32, end_x: i32);
    fn draw_range(start_x: i32, end_x: i32);

    fn change_cursor(value: &str);

    fn get_width() -> i32;
}

// TODO: Convert this to a trait to remove Wasm dependencies.
pub(crate) struct Drawing {}

impl Drawing {
    pub(crate) fn new() -> Self {
        Drawing {}
    }

    pub(crate) fn invalidate(&self) {
        invalidate();
    }

    pub(crate) fn draw_ongoing_range(&self, start: i32, end: i32) {
        draw_selected_range(start, end);
    }

    pub(crate) fn draw_ranges(&self, ranges: &Vec<Range>) {
        for range in ranges {
            draw_range(range.start, range.end);
        }
    }

    pub(crate) fn draw_ranges_with_selection(&self, ranges: &Vec<Range>, selected_index: usize) {
        for (index, range) in ranges.iter().enumerate() {
            if index == selected_index {
                draw_selected_range(range.start, range.end);
            } else {
                draw_range(range.start, range.end);
            }
        }
    }

    pub(crate) fn change_cursor(&self, value: &str) {
        change_cursor(value);
    }

    pub(crate) fn get_width(&self) -> i32 {
        get_width()
    }
}
