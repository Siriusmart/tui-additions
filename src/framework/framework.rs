use std::{error::Error, fmt::Display, io::Stdout};

use crossterm::event::KeyEvent;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use super::{
    CursorState, FrameworkClean, FrameworkData, FrameworkDirection, FrameworkHistory, ItemInfo,
    State,
};

/// Struct for a declarative TUI framework
///
/// Copy & paste examples can be found
/// [here](https://github.com/siriusmart/tui-additions/tree/master/examples/framework)

#[derive(Clone)]
pub struct Framework {
    /// Selectable items, auto generated when `state` is set with `new()` or `set_state()`
    pub selectables: Vec<Vec<(usize, usize)>>,
    /// Global data store for the framework
    pub data: FrameworkData,
    /// Defines the layout of items on screen
    pub state: State,
    /// The state and position of cursor
    pub cursor: CursorState,
    /// Stores saved states
    pub history: Vec<FrameworkHistory>,
}

impl Framework {
    /// Clears `self.history`
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Save current state
    pub fn push_history(&mut self) {
        self.history.push(FrameworkHistory {
            selectables: self.selectables.clone(),
            data: self.data.state.clone(),
            state: self.state.clone(),
            cursor: self.cursor.clone(),
        });
    }

    /// Removes the last history and returns it
    pub fn pop_history(&mut self) -> Option<FrameworkHistory> {
        self.history.pop()
    }

    /// Revert self to last save (if there is)
    pub fn revert_last_history(&mut self) -> Result<(), FrameworkError> {
        let history = match self.history.pop() {
            None => return Err(FrameworkError::NoSuchSave),
            Some(history) => history,
        };

        self.selectables = history.selectables;
        self.data.state = history.data;
        self.state = history.state;
        self.cursor = history.cursor;

        Ok(())
    }

    /// Revert self to history at index
    pub fn revert_history(&mut self, index: usize) -> Result<(), FrameworkError> {
        if index >= self.history.len() {
            return Err(FrameworkError::NoSuchSave);
        }

        let history = self.history.remove(index);

        self.selectables = history.selectables;
        self.data.state = history.data;
        self.state = history.state;
        self.cursor = history.cursor;

        Ok(())
    }
}

impl Framework {
    pub fn is_selected(&self) -> bool {
        self.cursor.is_selected()
    }

    pub fn is_hover(&self) -> bool {
        self.cursor.is_hover()
    }

    pub fn is_none(&self) -> bool {
        self.cursor.is_none()
    }
}

impl Framework {
    /// Create a new Framework struct
    pub fn new(state: State) -> Self {
        Self {
            selectables: state.selectables(),
            data: FrameworkData::default(),
            state,
            cursor: CursorState::default(),
            history: Vec::new(),
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

        let chunks = self.state.get_chunks(area);

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
        let (mut frameworkclean, state) = self.split_clean();

        for (y, (row, row_chunks)) in state.0.iter_mut().zip(chunks.iter()).enumerate() {
            for (x, (row_item, item_chunk)) in
                row.items.iter_mut().zip(row_chunks.iter()).enumerate()
            {
                row_item.item.render(
                    frame,
                    &mut frameworkclean,
                    *item_chunk,
                    // Some((x, y)) == selected,
                    // Some((x, y)) == hover,
                    popup_render,
                    ItemInfo {
                        selected: Some((x, y)) == selected,
                        hover: Some((x, y)) == hover,
                        x,
                        y,
                    },
                );
            }
        }
    }

    /// Render only one item
    pub fn render_only(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, x: usize, y: usize) {
        let chunk = self.state.get_chunks(frame.size())[y][x];

        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);

        self.render_only_raw(frame, x, y, chunk, false, selected, hover);
        self.render_only_raw(frame, x, y, chunk, true, selected, hover);
    }

    /// Render multiple items
    ///
    /// Location is in a format of `Vec<(x, y)>`
    pub fn render_only_multiple(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        locations: &Vec<(usize, usize)>,
    ) {
        let chunks = self.state.get_chunks(frame.size());

        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);

        locations.iter().for_each(|(x, y)| {
            self.render_only_raw(frame, *x, *y, chunks[*y][*x], false, selected, hover);
        });

        locations.iter().for_each(|(x, y)| {
            self.render_only_raw(frame, *x, *y, chunks[*y][*x], true, selected, hover);
        });
    }

    /// Render only with more controls
    pub fn render_only_raw(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        x: usize,
        y: usize,
        chunk: Rect,
        popup_render: bool,
        selected: Option<(usize, usize)>,
        hover: Option<(usize, usize)>,
    ) {
        let (mut frameworkclean, state) = self.split_clean();
        state.get_mut(x, y).render(
            frame,
            &mut frameworkclean,
            chunk,
            popup_render,
            ItemInfo {
                selected: selected == Some((x, y)),
                hover: hover == Some((x, y)),
                x,
                y,
            },
        )
    }

    /// Send key input to selected object, returns an `Err(())` when no objct is selected
    pub fn key_input(&mut self, key: KeyEvent) {
        let selected = self.cursor.selected(&self.selectables);
        let (mut frameworkclean, state) = self.split_clean();

        if let Some((x, y)) = selected {
            state.get_mut(x, y).key_event(
                &mut frameworkclean,
                key,
                ItemInfo {
                    selected: true,
                    hover: false,
                    x,
                    y,
                },
            );
        }
    }

    pub fn load(&mut self) {
        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);
        let (mut frameworkclean, state) = self.split_clean();

        for (y, row) in state.0.iter_mut().enumerate() {
            for (x, row_item) in row.items.iter_mut().enumerate() {
                row_item.item.load_item(
                    &mut frameworkclean,
                    ItemInfo {
                        selected: Some((x, y)) == selected,
                        hover: Some((x, y)) == hover,
                        x,
                        y,
                    },
                );
            }
        }
    }

    pub fn load_only(&mut self, x: usize, y: usize) {
        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);
        let (mut frameworkclean, state) = self.split_clean();

        state.get_mut(x, y).load_item(
            &mut frameworkclean,
            ItemInfo {
                selected: Some((x, y)) == selected,
                hover: Some((x, y)) == hover,
                x,
                y,
            },
        );
    }

    pub fn load_only_multiple(&mut self, locations: &Vec<(usize, usize)>) {
        let selected = self.cursor.selected(&self.selectables);
        let hover = self.cursor.hover(&self.selectables);
        let (mut frameworkclean, state) = self.split_clean();

        locations.iter().for_each(|(x, y)| {
            state.get_mut(*x, *y).load_item(
                &mut frameworkclean,
                ItemInfo {
                    selected: Some((*x, *y)) == selected,
                    hover: Some((*x, *y)) == hover,
                    x: *x,
                    y: *y,
                },
            );
        })
    }
}

impl Framework {
    /// Split `Framework` into `FrameworkClean` and `&mut State`
    pub fn split_clean(&mut self) -> (FrameworkClean, &mut State) {
        self.into()
    }
}

impl Framework {
    /// Move cursor in corresponding direction, will return an `Err(E)` if something is selected
    /// and the cursor is not free to move around
    pub fn r#move(&mut self, direction: FrameworkDirection) -> Result<(), FrameworkError> {
        self.cursor.r#move(direction, &self.selectables)
    }

    /// Select the hovering item
    pub fn select(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some((x, y)) = self.cursor.hover(&self.selectables) {
            let (mut frameworkclean, state) = self.split_clean();
            let item = state.get_mut(x, y);
            if item.select(&mut frameworkclean) {
                self.cursor.select()?;
            }
        } else {
            Err(FrameworkError::CursorStateMismatch)?;
        }

        Ok(())
    }

    /// Deselect the hovering item
    pub fn deselect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some((x, y)) = self.cursor.selected(&self.selectables) {
            let (mut frameworkclean, state) = self.split_clean();
            let item = state.get_mut(x, y);
            if item.deselect(&mut frameworkclean) {
                self.cursor.deselect()?;
            }
        } else {
            Err(FrameworkError::CursorStateMismatch)?;
        }

        Ok(())
    }
}

/// Errors that may be returned by `Framework`
#[derive(Debug)]
pub enum FrameworkError {
    /// Moving the cursor when something is selected (not allowed)
    MoveSelected,
    /// Calling `self.select()` when not hovering and `self.deselect()` when nothing is selected
    CursorStateMismatch,
    /// Not found in `self.history`, caused by incorrect index or `self.history` is empty
    NoSuchSave,
}

impl Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for FrameworkError {}
