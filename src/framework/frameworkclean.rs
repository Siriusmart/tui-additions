use super::{CursorState, Framework, State};
use typemap::CloneMap;

/// A version of `Framework` that does not include `State` and everything is a mutable reference
pub struct FrameworkClean<'a> {
    pub selectables: &'a mut Vec<Vec<(usize, usize)>>,
    pub data: &'a mut CloneMap,
    pub cursor: &'a mut CursorState,
}

impl<'a> Into<(FrameworkClean<'a>, &'a mut State)> for &'a mut Framework {
    fn into(self) -> (FrameworkClean<'a>, &'a mut State) {
        let state = &mut self.state;
        let frameworkclean = FrameworkClean {
            selectables: &mut self.selectables,
            data: &mut self.data,
            cursor: &mut self.cursor,
        };

        (frameworkclean, state)
    }
}
