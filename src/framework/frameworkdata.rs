use typemap::{CloneMap, TypeMap};

#[derive(Clone)]
pub struct FrameworkData {
    pub global: CloneMap,
    pub state: CloneMap,
}

impl Default for FrameworkData {
    fn default() -> Self {
        Self {
            global: TypeMap::custom(),
            state: TypeMap::custom(),
        }
    }
}

impl From<(CloneMap, CloneMap)> for FrameworkData {
    fn from((global, state): (CloneMap, CloneMap)) -> Self {
        Self { global, state }
    }
}
