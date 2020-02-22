use crate::drawing::Drawing;
use crate::state::*;
use crate::{Point2D, Range, VirtualKey};

pub(crate) struct Model {
    current_state: Box<dyn State>,
    drawing: Drawing,
    ranges: Vec<Range>,
}

pub(crate) struct Context<'model> {
    pub(crate) drawing: &'model mut Drawing,
    pub(crate) ranges: &'model mut Vec<Range>,
}

impl<'model> Context<'model> {
    pub(crate) fn redraw(&mut self) {
        self.drawing.invalidate();
        self.drawing.draw_ranges(self.ranges);
    }

    pub(crate) fn redraw_with_selection(&mut self, selected_index: usize) {
        self.drawing.invalidate();
        self.drawing
            .draw_ranges_with_selection(self.ranges, selected_index);
    }

    /// Adds a new range and returns the index of the range.
    // TODO: Reconsider
    pub(crate) fn add_range(&mut self, range: Range) -> usize {
        self.ranges.push(range);
        self.redraw();
        self.ranges.len() - 1
    }

    // TODO: Reconsider.
    pub(crate) fn get_range_by_index(&mut self, index: usize) -> &mut Range {
        &mut self.ranges[index]
    }

    // TODO: Reconsider.
    pub(crate) fn delete_range_by_index(&mut self, index: usize) {
        self.ranges.remove(index);
        self.redraw();
    }
}

fn create_context<'m>(drawing: &'m mut Drawing, ranges: &'m mut Vec<Range>) -> Context<'m> {
    Context { drawing, ranges }
}

impl Model {
    pub(crate) fn new() -> Self {
        Model {
            current_state: Box::new(IdleState::default()),
            ranges: Vec::new(),
            drawing: Drawing::new(),
        }
    }

    pub fn on_mouse_down(&mut self, cursor: Point2D) {
        let mut ctx = create_context(&mut self.drawing, &mut self.ranges);
        let transition = self.current_state.on_mouse_down(cursor, &mut ctx);
        self.update_state(transition);
    }

    pub fn on_mouse_up(&mut self, cursor: Point2D) {
        let mut ctx = create_context(&mut self.drawing, &mut self.ranges);
        let transition = self.current_state.on_mouse_up(cursor, &mut ctx);
        self.update_state(transition);
    }

    pub fn on_mouse_move(&mut self, cursor: Point2D) {
        let mut ctx = create_context(&mut self.drawing, &mut self.ranges);
        let transition = self.current_state.on_mouse_move(cursor, &mut ctx);
        self.update_state(transition);
    }

    pub fn on_key_up(&mut self, key: VirtualKey) {
        let mut ctx = create_context(&mut self.drawing, &mut self.ranges);
        let transition = self.current_state.on_key_up(key, &mut ctx);
        self.update_state(transition);
    }

    #[inline]
    fn update_state(&mut self, transition: Transition) {
        if let Some(state) = transition {
            info!("{:?} -> {:?}", self.current_state, state);
            self.current_state = state;
        }
    }
}
