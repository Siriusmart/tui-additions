use typemap::{CloneMap, TypeMap};

use super::{CursorState, Framework, FrameworkData, State};

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

impl From<FrameworkHistory> for Framework {
    fn from(original: FrameworkHistory) -> Framework {
        Framework {
            selectables: original.selectables,
            data: FrameworkData::from((TypeMap::custom(), original.data)),
            state: original.state,
            cursor: original.cursor,
            history: Vec::new(),
        }
    }
}
