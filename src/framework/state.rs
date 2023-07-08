use super::{FrameworkError, FrameworkItem};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Contains an item
#[derive(Clone)]
pub struct RowItem {
    /// The actual item Boxed
    pub item: Box<dyn FrameworkItem>,
    /// Width of the item
    pub width: Constraint,
}

/// Contains a row of objects
#[derive(Clone)]
pub struct Row {
    /// All the items in the row
    pub items: Vec<RowItem>,
    /// If the row should be centered or not
    pub centered: bool,
    /// The height of the row
    pub height: Constraint,
}

/// Contains the items and the layout of the TUI
#[derive(Clone)]
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
            if !row_selectables.is_empty() {
                selectables.push(row_selectables);
            }
        });

        selectables
    }

    /// Return chunks as 2D array of rects
    pub fn get_chunks(&self, area: Rect) -> Vec<Vec<Rect>> {
        // chunks
        let mut row_constraints = vec![Constraint::Length(0)];
        row_constraints.extend(self.0.iter().map(|row| row.height));
        row_constraints.push(Constraint::Length(0));

        let row_constraints_length = row_constraints.len() - 2;

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area)
            .iter()
            .skip(1)
            .take(row_constraints_length)
            .zip(
                self.0
                    .iter()
                    .map(|row| {
                        let begin_length = if row.centered {
                            Constraint::Length(
                                (area.width
                                    - row
                                        .items
                                        .iter()
                                        .map(|item| item.width.apply(area.width))
                                        .sum::<u16>())
                                    / 2,
                            )
                        } else {
                            Constraint::Length(0)
                        };

                        let mut out = vec![begin_length];
                        out.extend(row.items.iter().map(|item| item.width));
                        out.push(Constraint::Length(0));
                        out
                    }),
            )
            .map(|(row_chunk, constraints)| {
                let constraints_length = constraints.len() - 2;

                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(*row_chunk)
                    .iter()
                    .skip(1)
                    .take(constraints_length).copied()
                    .collect()
            })
            .collect::<Vec<_>>()
    }

    /// Get reference to item with x and y value
    pub fn get(&self, x: usize, y: usize) -> &dyn FrameworkItem {
        &*self.0[y].items[x].item
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
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected(_, _))
    }

    pub fn is_hover(&self) -> bool {
        matches!(self, Self::Hover(_, _))
    }

    pub fn is_none(&self) -> bool {
        self == &Self::None
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
            Self::Hover(x, y) if !selectables.is_empty() => {
                Some(Self::selectables_to_coors(selectables, (*x, *y)))
            }
            _ => None,
        }
    }

    pub fn selected(&self, selectables: &Vec<Vec<(usize, usize)>>) -> Option<(usize, usize)> {
        match self {
            Self::Selected(x, y) if !selectables.is_empty() => {
                Some(Self::selectables_to_coors(selectables, (*x, *y)))
            }
            _ => None,
        }
    }

    fn selectables_to_coors(
        selectables: &[Vec<(usize, usize)>],
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
        direction: FrameworkDirection,
        selectables: &Vec<Vec<(usize, usize)>>,
    ) -> Result<(), FrameworkError> {
        match direction {
            FrameworkDirection::Up => self.up(),
            FrameworkDirection::Down => self.down(),
            FrameworkDirection::Left => self.left(),
            FrameworkDirection::Right => self.right(),
        }?;

        self.move_check(selectables);

        Ok(())
    }

    fn move_check(&mut self, selectables: &Vec<Vec<(usize, usize)>>) {
        if let Self::Hover(x, y) = self {
            if selectables.is_empty() {
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

/// Used to represent direction in this crate
#[derive(Clone, Copy)]
pub enum FrameworkDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Passed into the `FrameworkItem` trait functions for info of the item
#[derive(Clone, Copy)]
pub struct ItemInfo {
    pub selected: bool,
    pub hover: bool,
    pub x: usize,
    pub y: usize,
}
