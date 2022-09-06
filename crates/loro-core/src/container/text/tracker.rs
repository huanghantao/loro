use crate::{op::Op, span::IdSpan, VersionVector};

use self::cursor_map::CursorMap;

mod content_map;
mod cursor_map;
mod y_span;

struct Tracker {
    index: CursorMap,
}

impl Tracker {
    fn turn_on(&mut self, _id: IdSpan) {}
    fn turn_off(&mut self, _id: IdSpan) {}
    fn checkout(&mut self, _vv: VersionVector) {}
    fn apply(&mut self, _content: &Op) {}
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_tracker() -> Tracker {
        Tracker {
            index: Default::default(),
        }
    }

    #[test]
    fn test_turn_off() {
        let mut tracker = create_tracker();
        tracker.turn_off(IdSpan::new(1, 1, 2));
    }
}