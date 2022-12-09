use tui::layout::Rect;

use super::{CursorState, Framework, FrameworkData, State};

/// A version of `Framework` that does not include `State` and everything is a mutable reference
pub struct FrameworkClean<'a> {
    pub selectables: &'a mut Vec<Vec<(usize, usize)>>,
    pub data: &'a mut FrameworkData,
    pub cursor: &'a mut CursorState,
    pub frame_area: &'a mut Option<Rect>,
}

impl<'a> From<&'a mut Framework> for (FrameworkClean<'a>, &'a mut State) {
    fn from(original: &'a mut Framework) -> (FrameworkClean<'a>, &'a mut State) {
        let state = &mut original.state;
        let frameworkclean = FrameworkClean {
            selectables: &mut original.selectables,
            data: &mut original.data,
            cursor: &mut original.cursor,
            frame_area: &mut original.frame_area,
        };

        (frameworkclean, state)
    }
}
