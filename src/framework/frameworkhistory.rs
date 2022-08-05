use typemap::{CloneMap, TypeMap};

use super::{CursorState, State, Framework, FrameworkData};

/// Save state for Framework
#[derive(Clone)]
pub struct FrameworkHistory {
    /// Selectable items, auto generated when `state` is set with `new()` or `set_state()`
    pub selectables: Vec<Vec<(usize, usize)>>,
    /// Global data store for the framework
    pub data: CloneMap,
    /// Defines the layout of items on screen
    pub state: State,
    /// The state and position of cursor
    pub cursor: CursorState,
}

impl Into<Framework> for FrameworkHistory {
    fn into(self) -> Framework {
        Framework {
            selectables: self.selectables,
            data: FrameworkData::from((TypeMap::custom(), self.data)),
            state: self.state,
            cursor: self.cursor,
            history: Vec::new(),
        }
    }
}
