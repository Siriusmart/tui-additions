use std::fmt::Display;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};

#[derive(Clone)]
pub struct TextField {
    pub content: String,
    pub scroll: usize,
    pub cursor: usize,
    pub style: Style,
    pub text_style: Style,
    pub cursor_style: Style,
    pub width: Option<u16>,
}

impl Widget for TextField {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        if let Some(width) = self.width {
            if width != area.width {
                panic!("width mismatch");
            }
        } else {
            panic!("unknown width");
        }

        let cursor_at_end = self.cursor == self.content.len();
        let mut spans = vec![Span::styled(
            self.content[self.scroll..self.cursor].to_string(),
            self.text_style,
        )];

        if cursor_at_end {
            spans.push(Span::styled(String::from(' '), self.cursor_style));
        } else {
            spans.push(Span::styled(
                self.content[self.cursor..self.cursor + 1].to_string(),
                self.cursor_style,
            ));
            spans.push(Span::styled(
                self.content[self.cursor + 1..self.content.len()].to_string(),
                self.text_style,
            ));
        }

        let paragraph = Paragraph::new(Spans::from(spans)).style(self.style);
        paragraph.render(area, buf);
    }
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            content: String::default(),
            scroll: 0,
            cursor: 0,
            style: Style::default(),
            text_style: Style::default(),
            cursor_style: Style::default().bg(Color::Gray),
            width: None,
        }
    }
}

impl TextField {
    pub fn insert(&mut self, index: usize, c: char) -> Result<(), TextFieldError> {
        self.content.insert(index, c);
        self.cursor += 1;
        self.update()?;
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<Option<char>, TextFieldError> {
        if self.cursor == 0 {
            return Ok(None);
        }
        let c = self.content.remove(index - 1);
        self.cursor -= 1;
        self.update()?;
        Ok(Some(c))
    }

    pub fn left(&mut self) -> Result<(), TextFieldError> {
        if self.cursor == 0 {
            return Ok(());
        }

        self.cursor -= 1;
        self.update()
    }

    pub fn right(&mut self) -> Result<(), TextFieldError> {
        if self.cursor == self.content.len() {
            return Ok(());
        }

        self.cursor += 1;
        self.update()
    }

    pub fn first(&mut self) -> Result<(), TextFieldError> {
        self.cursor = 0;
        self.update()
    }

    pub fn last(&mut self) -> Result<(), TextFieldError> {
        self.cursor = self.content.len();
        self.update()
    }
}

impl TextField {
    pub fn set_width(&mut self, width: u16) {
        self.width = Some(width)
    }

    pub fn update(&mut self) -> Result<(), TextFieldError> {
        let width = if let Some(width) = self.width {
            width
        } else {
            return Err(TextFieldError::UnknownWidth);
        };

        if self.scroll > self.cursor {
            self.scroll = self.cursor;
        } else if self.scroll + width as usize - 1 < self.cursor {
            self.scroll = self.cursor - width as usize + 1;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum TextFieldError {
    UnknownWidth,
}

impl Display for TextFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
