use std::{error::Error, fmt::Display};

use tui::{
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use unicode_segmentation::UnicodeSegmentation;

/// A widget for selecting from a list of items
///
/// Copy & paste examples can be found
/// [here](https://github.com/siriusmart/tui-additions/tree/master/examples/textlist)
///
/// The requirement for the text list widget to render are:
/// * Minimal height of 3
/// * Height should be updated with `self.set_height()` before rendering

#[derive(Clone)]
pub struct TextList {
    /// Items that are in the list, set by `.items()` or `.set_items()` function
    pub items: Vec<String>,
    /// The selected item, should be updated using provided functions. `0` should be the first item
    pub selected: usize,
    /// How many items to scroll down from the first item, should auto update if `selected` is
    /// changed with provided functions.
    pub scroll: usize,
    /// The style of the entire text list including unselected (normal) items
    pub style: Style,
    /// Cursor style is the style of the box around the selected item
    pub cursor_style: Style,
    /// Style of the selected item
    pub selected_style: Style,
    /// The border type of cursor
    pub border_type: BorderType,
    /// Height avaliable for the widget, should be updated before rendering the widget
    pub height: Option<u16>,
    /// Only allow ASCII characters to prevent unicode length issues
    pub ascii_only: bool,
    /// Character to replace non ASCII characters with, only useful when `ascii_only` is `true`
    pub non_ascii_replace: char,
    /// How to handle items that got a longer length than the width which the widget can render
    pub trim_type: TrimType,
}

/// Movement related functions
impl TextList {
    /// Should run this function after `scoll` of `selected` is updated to ensure that the cursor
    /// is on screen
    pub fn update(&mut self) -> Result<(), TextListError> {
        let height = if let Some(h) = self.height {
            h as i32 - 2
        } else {
            return Err(TextListError::UnknownHeight);
        };

        if height <= 0 {
            return Err(TextListError::NotEnoughHeight);
        }

        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.scroll + height as usize <= self.selected {
            self.scroll = self.selected - height as usize + 1;
        }
        Ok(())
    }

    /// Move cursor up by 1 item (if there is)
    pub fn up(&mut self) -> Result<(), TextListError> {
        if self.selected != 0 {
            self.selected -= 1;
            self.update()?;
        }
        Ok(())
    }

    /// Move cursor down by 1 item (if there is)
    pub fn down(&mut self) -> Result<(), TextListError> {
        if self.items.len() == 0 {
            return Ok(());
        }

        if self.selected < self.items.len() - 1 {
            self.selected += 1;
            self.update()?;
        }
        Ok(())
    }

    /// Go up 1 page without changing the cursor position on screen
    pub fn pageup(&mut self) -> Result<(), TextListError> {
        let height = match self.height {
            Some(h) => h as usize,
            None => return Err(TextListError::UnknownHeight),
        };

        if self.selected == 0 {
            return Ok(());
        }

        let shift_by = height - 2;

        if self.selected < shift_by {
            self.selected = 0;
        } else {
            self.selected -= shift_by;

            if self.scroll > shift_by {
                self.scroll -= shift_by;
            } else {
                self.scroll = 0;
            }
        }

        self.update()?;

        Ok(())
    }

    /// Go down 1 page without changing the cursor position on screen
    pub fn pagedown(&mut self) -> Result<(), TextListError> {
        let height = match self.height {
            Some(h) => h as usize,
            None => return Err(TextListError::UnknownHeight),
        };

        if self.selected >= self.items.len() - 1 {
            return Ok(());
        }

        let shift_by = height - 2;

        if self.selected + shift_by > self.items.len() - 1 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected += shift_by;

            if self.scroll + shift_by + height - 2 < self.items.len() {
                self.scroll += shift_by;
            } else {
                self.scroll = self.items.len() - 1 - height + 2;
            }
        }

        self.update()?;

        Ok(())
    }

    /// Go to the first item
    pub fn first(&mut self) -> Result<(), TextListError> {
        if self.selected == 0 {
            return Ok(());
        }

        self.selected = 0;
        self.update()?;
        Ok(())
    }

    /// Go to the last item
    pub fn last(&mut self) -> Result<(), TextListError> {
        if self.selected == self.items.len() - 1 {
            return Ok(());
        }

        self.selected = self.items.len() - 1;
        self.update()?;
        Ok(())
    }
}

/// Setters
///
/// * `set_{feature}()` takes ownership of self and returns self
/// * `{feature}()` takes a mutable reference to self
impl TextList {
    pub fn ascii_only(mut self, ascii_only: bool) -> Self {
        self.set_ascii_only(ascii_only);
        self
    }

    pub fn set_ascii_only(&mut self, ascii_only: bool) {
        self.ascii_only = ascii_only;
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.set_border_type(border_type);
        self
    }

    pub fn set_border_type(&mut self, border_type: BorderType) {
        self.border_type = border_type;
    }

    pub fn cursor_style(mut self, cursor_style: Style) -> Self {
        self.set_cursor_style(cursor_style);
        self
    }

    pub fn set_cursor_style(&mut self, cursor_style: Style) {
        self.cursor_style = cursor_style;
    }

    pub fn height(mut self, height: u16) -> Self {
        self.set_height(height);
        self
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = Some(height);
    }

    pub fn items<D: Display>(mut self, items: &Vec<D>) -> Result<Self, Box<dyn Error>> {
        self.set_items(items)?;
        Ok(self)
    }

    pub fn set_items<D: Display>(&mut self, items: &Vec<D>) -> Result<(), Box<dyn Error>> {
        self.items = items.iter().map(|item| format!("{}", item)).collect();
        if self.height.is_some() {
            self.update()?;
        }
        Ok(())
    }

    pub fn selected(mut self, index: usize) -> Result<Self, TextListError> {
        self.set_selected(index)?;
        Ok(self)
    }

    pub fn set_selected(&mut self, index: usize) -> Result<(), TextListError> {
        self.selected = index;
        self.update()?;
        Ok(())
    }

    pub fn non_ascii_replace(mut self, non_ascii_replace: char) -> Self {
        self.set_non_ascii_replace(non_ascii_replace);
        self
    }

    pub fn set_non_ascii_replace(&mut self, non_ascii_replace: char) {
        self.non_ascii_replace = non_ascii_replace;
    }

    pub fn selected_style(mut self, selected_style: Style) -> Self {
        self.set_selected_style(selected_style);
        self
    }

    pub fn set_selected_style(&mut self, selected_style: Style) {
        self.selected_style = selected_style;
    }

    pub fn style(mut self, style: Style) -> Self {
        self.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn trim_type(mut self, trim_type: TrimType) -> Self {
        self.set_trim_type(trim_type);
        self
    }

    pub fn set_trim_type(&mut self, trim_type: TrimType) {
        self.trim_type = trim_type;
    }
}

/// Default (blank) text list
impl Default for TextList {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            scroll: 0,
            style: Style::default(),
            cursor_style: Style::default(),
            selected_style: Style::default(),
            border_type: BorderType::Plain,
            height: None,
            ascii_only: false,
            non_ascii_replace: '?',
            trim_type: TrimType::FullTripleDot,
        }
    }
}

/// `tui::widget::Widget` implementation
impl Widget for TextList {
    /// Note that if `self.height` does not match the actualy height, it will panic instead because
    /// there is no way to return a `Result<T, E>` out of this function
    fn render(mut self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let height = self.height.expect("unknown height");
        if height != area.height {
            panic!("height mismatch");
        }

        if area.height < 3 {
            // panic!("insufficient height");
            return;
        }

        self.items = self
            .items
            .into_iter()
            .skip(self.scroll)
            .take(height as usize - 2)
            .collect();

        // remove non ascii character

        if self.ascii_only {
            self.items.iter_mut().for_each(|item| {
                *item = item
                    .chars()
                    .map(|c| {
                        if c.is_ascii() {
                            c
                        } else {
                            self.non_ascii_replace
                        }
                    })
                    .collect();
            });
        }

        // check if item is too long

        let width_from = area.width as usize - 2;
        let (width_after, end_with) = match self.trim_type {
            TrimType::None => (width_from, ""),
            TrimType::FullTripleDot => (width_from - 3, "..."),
            TrimType::ShortTripleDot => (width_from - 1, "…"),
        };

        if area.width as usize - 2 < end_with.chars().count() {
            panic!("width too small");
        }

        self.items.iter_mut().for_each(|item| {
            let chars = UnicodeSegmentation::graphemes(item.as_str(), true).collect::<Vec<_>>();
            if chars.len() > width_from {
                *item = format!("{}{}", chars.into_iter().take(width_after).collect::<String>(), end_with);
            }
        });

        // setting background style for rect

        buf.set_style(area, self.style);

        // render items

        let mut y = area.y;
        self.items
            .into_iter()
            .zip(self.scroll..)
            .for_each(|(item, index)| {
                if index == self.selected {
                    let block = Block::default()
                        .border_type(self.border_type)
                        .border_style(self.cursor_style)
                        .borders(Borders::ALL);
                    let paragraph = Paragraph::new(item).style(self.selected_style).block(block);

                    let select_area = Rect {
                        x: area.x,
                        y,
                        height: 3,
                        width: area.width,
                    };

                    paragraph.render(select_area, buf);
                    y += 3;
                } else {
                    buf.set_string(area.x + 1, y, item, Style::default());
                    y += 1;
                }
            })
    }
}

/// Errors that the text list functions may return
#[derive(Debug)]
pub enum TextListError {
    /// `self.height` is not initialized (is_none)
    UnknownHeight,
    /// Not enough height to draw the text list widget (the minimal height is 3)
    NotEnoughHeight,
}

impl Display for TextListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl Error for TextListError {}

/// How to handle items that are longer than the avaliable width
#[derive(Debug, Clone, Copy)]
pub enum TrimType {
    /// Add `'…'` to the end of item
    ShortTripleDot,
    /// Add `'...'` to the end of item
    FullTripleDot,
    /// Add nothing to the end of item
    r#None,
}
