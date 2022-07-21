use std::{error::Error, fmt::Display, io::Stdout};

use crossterm::event::KeyEvent;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use typemap::TypeMap;

use super::{CursorState, Direction as MoveDirection, FrameworkClean, State};

/// Struct for a declarative TUI framework
///
/// Copy & paste examples can be found
/// [here](https://github.com/siriusmart/tui-additions/tree/master/examples/framework)

pub struct Framework {
    /// Selectable items, auto generated when `state` is set with `new()` or `set_state()`
    pub selectables: Vec<Vec<(usize, usize)>>,
    /// Global data store for the framework
    pub data: TypeMap,
    /// Defines the layout of items on screen
    pub state: State,
    /// The state and position of cursor
    pub cursor: CursorState,
}

impl Framework {
    /// Create a new Framework struct
    pub fn new(state: State) -> Self {
        Self {
            selectables: state.selectables(),
            data: TypeMap::new(),
            state,
            cursor: CursorState::default(),
        }
    }

    /// Set `self.state` and also update `self.selectables`
    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.selectables = self.state.selectables();
    }

    /// Render every item to screen
    pub fn render(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let area = frame.size();

        // chunks
        let mut row_constraints = vec![Constraint::Length(0)];
        row_constraints.extend(self.state.0.iter().map(|row| row.height));
        row_constraints.push(Constraint::Length(0));

        let row_constraints_length = row_constraints.len() - 2;

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area)
            .into_iter()
            .skip(1)
            .take(row_constraints_length)
            .collect::<Vec<_>>();

        let constraints = self
            .state
            .0
            .iter()
            .map(|row| {
                let begin_length = if row.centered {
                    Constraint::Length(
                        (area.width
                            - row
                                .items
                                .iter()
                                .map(|item| Self::constraint_to_length(item.width, area.width))
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
            })
            .collect::<Vec<_>>();

        let chunks = rows
            .into_iter()
            .zip(constraints.into_iter())
            .map(|(row_chunk, constraints)| {
                let constraints_length = constraints.len() - 2;

                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(row_chunk)
                    .into_iter()
                    .skip(1)
                    .take(constraints_length)
                    .collect()
            })
            .collect::<Vec<_>>();

        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);

        // actually rendering the stuff
        self.render_raw(frame, &chunks, selected, hover, false);
        self.render_raw(frame, &chunks, selected, hover, true);
    }

    /// Render to screen with more controls
    pub fn render_raw(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        chunks: &Vec<Vec<Rect>>,
        selected: Option<(usize, usize)>,
        hover: Option<(usize, usize)>,
        popup_render: bool,
    ) {
        let (frameworkclean, state) = self.split_clean();

        for (y, (row, row_chunks)) in state.0.iter_mut().zip(chunks.iter()).enumerate() {
            for (x, (row_item, item_chunk)) in
                row.items.iter_mut().zip(row_chunks.iter()).enumerate()
            {
                row_item.item.render(
                    frame,
                    &frameworkclean,
                    *item_chunk,
                    Some((x, y)) == selected,
                    Some((x, y)) == hover,
                    popup_render,
                );
            }
        }
    }

    /// Send key input to selected object, returns an `Err(())` when no objct is selected
    pub fn key_input(&mut self, key: KeyEvent) -> Result<(), ()> {
        let selected = self.cursor.selected(&self.selectables);
        let (frameworkclean, state) = self.split_clean();

        if let Some((x, y)) = selected {
            state.get_mut(x, y).key_event(frameworkclean, key);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Split `Framework` into `FrameworkClean` and `&mut State`
    pub fn split_clean(&mut self) -> (FrameworkClean<'_>, &mut State) {
        self.into()
    }

    fn constraint_to_length(constraint: Constraint, length_to_split: u16) -> u16 {
        match constraint {
            Constraint::Min(width) | Constraint::Length(width) => width,
            Constraint::Percentage(percentage) => length_to_split * percentage / 100,
            _ => unimplemented!("max or ration cannot be converted to a fixed length"),
        }
    }
}

impl Framework {
    /// Move cursor in corresponding direction, will return an `Err(E)` if something is selected
    /// and the cursor is not free to move around
    pub fn r#move(&mut self, direction: MoveDirection) -> Result<(), FrameworkError> {
        self.cursor.r#move(direction, &self.selectables)
    }

    /// Select the hovering item
    pub fn select(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some((x, y)) = self.cursor.hover(&self.selectables) {
            let (frameworkclean, state) = self.split_clean();
            let item = state.get_mut(x, y);
            item.select(&frameworkclean)?;
            self.cursor.select()?;
        } else {
            Err(FrameworkError::CursorStateMismatch)?;
        }

        Ok(())
    }

    /// Deselect the hovering item
    pub fn deselect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some((x, y)) = self.cursor.selected(&self.selectables) {
            let (frameworkclean, state) = self.split_clean();
            let item = state.get_mut(x, y);
            item.deselect(&frameworkclean)?;
            self.cursor.deselect()?;
        } else {
            Err(FrameworkError::CursorStateMismatch)?;
        }

        Ok(())
    }
}

/// Errors that may be returned by `Framework`
#[derive(Debug)]
pub enum FrameworkError {
    MoveSelected,
    CursorStateMismatch,
}

impl Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for FrameworkError {}
