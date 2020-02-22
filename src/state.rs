use crate::model::Context;
use crate::{Point2D, Range, VirtualKey};

pub(crate) type Transition = Option<Box<dyn State>>;

pub(crate) trait State: std::fmt::Debug {
    fn on_mouse_down(&mut self, _cursor: Point2D, _ctx: &mut Context) -> Transition {
        None
    }

    fn on_mouse_up(&mut self, _cursor: Point2D, _ctx: &mut Context) -> Transition {
        None
    }

    fn on_mouse_move(&mut self, _cursor: Point2D, _ctx: &mut Context) -> Transition {
        None
    }

    fn on_key_up(&mut self, _key: VirtualKey, _xtx: &mut Context) -> Transition {
        None
    }
}

// Idle state

const RANGE_EDGE_MARGIN: i32 = 8;

fn get_left_bound(pos: i32, ctx: &mut Context) -> i32 {
    // TODO: Optimize
    let mut left_bound = 0;
    for range in ctx.ranges.iter() {
        if range.end > pos {
            continue;
        }
        left_bound = std::cmp::max(left_bound, range.end);
    }
    left_bound
}

fn get_right_bound(pos: i32, ctx: &mut Context) -> i32 {
    // TODO: Optimize
    let mut right_bound = ctx.drawing.get_width();
    for range in ctx.ranges.iter() {
        if range.start < pos {
            continue;
        }
        right_bound = std::cmp::min(right_bound, range.start);
    }
    right_bound
}

fn update_range_start(pos: i32, index: usize, ctx: &mut Context) -> Transition {
    let left_bound = get_left_bound(pos, ctx);
    let right_bound = ctx.get_range_by_index(index).end;
    let bounds = Range::new(left_bound, right_bound);
    Some(Box::new(RangeUpdateState::new(
        bounds,
        index,
        Side::Start,
        pos,
    )))
}

fn update_range_end(pos: i32, index: usize, ctx: &mut Context) -> Transition {
    let left_bound = ctx.get_range_by_index(index).start;
    let right_bound = get_right_bound(pos, ctx);
    let bounds = Range::new(left_bound, right_bound);
    Some(Box::new(RangeUpdateState::new(
        bounds,
        index,
        Side::End,
        pos,
    )))
}

fn hit_test(pos: i32, ctx: &mut Context) -> Transition {
    // TODO: Check if we could make faster than O(N).
    let mut start = 0;
    let mut end = ctx.drawing.get_width();
    for (index, range) in ctx.ranges.iter().enumerate() {
        let range_start_with_margin = std::cmp::max(0, range.start - RANGE_EDGE_MARGIN);
        let range_end_with_margin = std::cmp::min(end, range.end + RANGE_EDGE_MARGIN);
        if range_start_with_margin <= pos && range_end_with_margin >= pos {
            // Found a range that contains `pos`.
            if pos < range.start + RANGE_EDGE_MARGIN {
                return update_range_start(pos, index, ctx);
            } else if pos > range.end - RANGE_EDGE_MARGIN {
                return update_range_end(pos, index, ctx);
            } else {
                // `pos` is in a range.
                ctx.redraw_with_selection(index);
                return Some(Box::new(RangeSelectedState::new(index)));
            }
        }
        if range.end < pos {
            start = std::cmp::max(start, range.end);
        }
        if range.start > pos {
            end = std::cmp::min(end, range.start);
        }
    }

    if start <= pos && end >= pos {
        let bounds = Range::new(start, end);
        Some(Box::new(NewRangeState::new(bounds, pos)))
    } else {
        None
    }
}

fn update_mouse_cursor(pos: i32, ctx: &mut Context) {
    // TODO: Optimize
    for range in ctx.ranges.iter() {
        let range_start_with_margin = std::cmp::max(0, range.start - RANGE_EDGE_MARGIN);
        let range_end_with_margin =
            std::cmp::min(ctx.drawing.get_width(), range.end + RANGE_EDGE_MARGIN);
        // |range| doesn't contain the point |cursor| points.
        if range_start_with_margin > pos || range_end_with_margin < pos {
            continue;
        }
        if pos < range.start + RANGE_EDGE_MARGIN {
            ctx.drawing.change_cursor("w-resize");
            return;
        } else if pos > range.end - RANGE_EDGE_MARGIN {
            ctx.drawing.change_cursor("e-resize");
            return;
        }
    }
    ctx.drawing.change_cursor("default");
}

#[derive(Default, Debug)]
pub(crate) struct IdleState {}

impl State for IdleState {
    fn on_mouse_move(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        update_mouse_cursor(cursor.x, ctx);
        None
    }

    fn on_mouse_down(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        hit_test(cursor.x, ctx)
    }
}

#[inline]
fn clamp(x: i32, bounds: &Range) -> i32 {
    if x < bounds.start {
        bounds.start
    } else if x > bounds.end {
        bounds.end
    } else {
        x
    }
}

// NewRangeState

#[derive(Debug)]
pub(crate) struct NewRangeState {
    bounds: Range,
    start_x: i32,
    end_x: i32,
}

impl NewRangeState {
    fn new(bounds: Range, start_x: i32) -> Self {
        assert!(start_x >= bounds.start && start_x <= bounds.end);
        NewRangeState {
            bounds,
            start_x,
            end_x: start_x,
        }
    }
}

impl State for NewRangeState {
    fn on_mouse_move(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        let new_end = clamp(cursor.x, &self.bounds);
        // TODO: Don't invalidate & redraw existing ranges.
        ctx.redraw();
        ctx.drawing.draw_ongoing_range(self.start_x, new_end);
        self.end_x = new_end;
        None
    }

    fn on_mouse_up(&mut self, _cursor: Point2D, ctx: &mut Context) -> Transition {
        // Don't create range if it's too small.
        if (self.end_x - self.start_x).abs() > RANGE_EDGE_MARGIN {
            let index = ctx.add_range(Range::new(self.start_x, self.end_x));
            ctx.redraw_with_selection(index);
            Some(Box::new(RangeSelectedState::new(index)))
        } else {
            ctx.redraw();
            Some(Box::new(IdleState::default()))
        }
    }
}

#[derive(Debug)]
enum Side {
    Start,
    End,
}

#[derive(Debug)]
pub(crate) struct RangeUpdateState {
    bounds: Range,
    index_in_ranges: usize,
    side: Side,
    new_x: i32,
}

impl RangeUpdateState {
    fn new(bounds: Range, index_in_ranges: usize, side: Side, start_x: i32) -> Self {
        let new_x = start_x;
        RangeUpdateState {
            bounds,
            index_in_ranges,
            side,
            new_x,
        }
    }
}

impl State for RangeUpdateState {
    fn on_mouse_move(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        let new_x = clamp(cursor.x, &self.bounds);
        let range = ctx.get_range_by_index(self.index_in_ranges);
        match self.side {
            Side::Start => range.start = new_x,
            Side::End => range.end = new_x,
        }
        // TODO: Redraw only the range being updating.
        ctx.redraw_with_selection(self.index_in_ranges);
        self.new_x = new_x;
        None
    }

    fn on_mouse_up(&mut self, _cursor: Point2D, _ctx: &mut Context) -> Transition {
        Some(Box::new(IdleState::default()))
    }
}

#[derive(Debug)]
pub(crate) struct RangeSelectedState {
    index_in_ranges: usize,
}

impl RangeSelectedState {
    fn new(index_in_ranges: usize) -> Self {
        RangeSelectedState { index_in_ranges }
    }

    fn deselect(&self, ctx: &mut Context) -> Transition {
        ctx.redraw();
        Some(Box::new(IdleState::default()))
    }
}

impl State for RangeSelectedState {
    fn on_mouse_move(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        update_mouse_cursor(cursor.x, ctx);
        None
    }

    fn on_mouse_down(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        hit_test(cursor.x, ctx)
    }

    fn on_mouse_up(&mut self, cursor: Point2D, ctx: &mut Context) -> Transition {
        let range = ctx.get_range_by_index(self.index_in_ranges);
        if range.start > cursor.x || range.end < cursor.x {
            self.deselect(ctx)
        } else {
            None
        }
    }

    fn on_key_up(&mut self, key: VirtualKey, ctx: &mut Context) -> Transition {
        match key {
            VirtualKey::Delete => {
                ctx.delete_range_by_index(self.index_in_ranges);
                Some(Box::new(IdleState::default()))
            }
            VirtualKey::Escape => self.deselect(ctx),
        }
    }
}
