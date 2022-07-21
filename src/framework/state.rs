use super::{FrameworkError, FrameworkItem};
use tui::layout::Constraint;

/// Contains an item
pub struct RowItem {
    /// The actual item Boxed
    pub item: Box<dyn FrameworkItem>,
    /// Width of the item
    pub width: Constraint,
}

/// Contains a row of objects
pub struct Row {
    /// All the items in the row
    pub items: Vec<RowItem>,
    /// If the row should be centered or not
    pub centered: bool,
    /// The height of the row
    pub height: Constraint,
}

/// Contains the items and the layout of the TUI
pub struct State(pub Vec<Row>);

impl State {
    /// Generate selectables which is a 2D vector of items that can be selected
    ///
    /// `(usize, usize)` maps to the `(x, y)` position to `State.0`, items that are not selectable
    /// are excluded
    pub fn selectables(&self) -> Vec<Vec<(usize, usize)>> {
        let mut selectables = Vec::new();

        self.0.iter().enumerate().for_each(|(y, row)| {
            let mut row_selectables = Vec::new();
            row.items.iter().enumerate().for_each(|(x, row_item)| {
                if row_item.item.selectable() {
                    row_selectables.push((x, y));
                }
            });
            if row_selectables.len() != 0 {
                selectables.push(row_selectables);
            }
        });

        selectables
    }

    /// Get reference to item with x and y value
    pub fn get(&self, x: usize, y: usize) -> &Box<dyn FrameworkItem> {
        &self.0[y].items[x].item
    }

    /// Get mutable reference to item with x and y value
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Box<dyn FrameworkItem> {
        &mut self.0[y].items[x].item
    }
}

/// State of cursor
///
/// The 2 numbers represent the x and y in `Framework.selectables` rather than `State.0`
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CursorState {
    /// Nothing is selected
    None,
    /// Cursor is hovering on an item
    Hover(usize, usize),
    /// An item is selected
    Selected(usize, usize),
}

impl Default for CursorState {
    fn default() -> Self {
        Self::None
    }
}

impl CursorState {
    /// Try select an item, will not work if an item is already selected or the cursor is not
    /// hovering on anything
    pub fn select(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Hover(x, y) => *self = Self::Selected(*x, *y),
            _ => return Err(FrameworkError::CursorStateMismatch),
        }

        Ok(())
    }

    /// Try deselect an item, will not work if no items are selected
    pub fn deselect(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Selected(x, y) => *self = Self::Hover(*x, *y),
            _ => return Err(FrameworkError::CursorStateMismatch),
        }

        Ok(())
    }
}

impl CursorState {
    pub fn to_hover(location: (usize, usize)) -> Self {
        Self::Hover(location.0, location.1)
    }

    pub fn to_selected(location: (usize, usize)) -> Self {
        Self::Selected(location.0, location.1)
    }

    pub fn hover(&self, selectables: &Vec<Vec<(usize, usize)>>) -> Option<(usize, usize)> {
        match self {
            Self::Hover(x, y) if selectables.len() != 0 => Some(Self::selectables_to_coors(&selectables, (*x, *y))),
            _ => None,
        }
    }

    pub fn selected(&self, selectables: &Vec<Vec<(usize, usize)>>) -> Option<(usize, usize)> {
        match self {
            Self::Selected(x, y) if selectables.len() != 0 => Some(Self::selectables_to_coors(&selectables, (*x, *y))),
            _ => None,
        }
    }

    fn selectables_to_coors(
        selectables: &Vec<Vec<(usize, usize)>>,
        location: (usize, usize),
    ) -> (usize, usize) {
        let (location_x, location_y) = location;

        selectables[location_y][location_x]
    }
}

impl CursorState {
    /// Move in the corresponding direction
    pub fn r#move(
        &mut self,
        direction: Direction,
        selectables: &Vec<Vec<(usize, usize)>>,
    ) -> Result<(), FrameworkError> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }?;

        self.move_check(selectables);

        Ok(())
    }

    fn move_check(&mut self, selectables: &Vec<Vec<(usize, usize)>>) {
        if let Self::Hover(x, y) = self {
            if selectables.len() == 0 {
                *x = 0;
                *y = 0;
                return;
            }
            let y_max = selectables.len() - 1;
            if *y > y_max {
                *y = y_max;
            }

            let x_max = selectables[*y].len() - 1;
            if *x > x_max {
                *x = x_max;
            }
        } else {
            unreachable!("move_check is only called after a hovering cursor is moved, when cursor is at hover state")
        }
    }

    fn left(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Hover(x, _) => {
                if *x != 0 {
                    *x -= 1
                }
            }
            Self::None => *self = Self::Hover(0, 0),
            Self::Selected(_, _) => return Err(FrameworkError::MoveSelected),
        }

        Ok(())
    }

    fn right(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Hover(x, _) => *x += 1,
            Self::None => *self = Self::Hover(usize::MAX, 0),
            Self::Selected(_, _) => return Err(FrameworkError::MoveSelected),
        }

        Ok(())
    }

    fn up(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Hover(_, y) => {
                if *y != 0 {
                    *y -= 1
                }
            }
            Self::None => *self = Self::Hover(0, 0),
            Self::Selected(_, _) => return Err(FrameworkError::MoveSelected),
        }

        Ok(())
    }

    fn down(&mut self) -> Result<(), FrameworkError> {
        match self {
            Self::Hover(_, y) => *y += 1,
            Self::None => *self = Self::Hover(0, usize::MAX),
            Self::Selected(_, _) => return Err(FrameworkError::MoveSelected),
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
