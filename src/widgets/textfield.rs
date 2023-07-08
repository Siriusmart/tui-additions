use std::fmt::Display;

use ratatui::{
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Paragraph, Widget},
};
use unicode_segmentation::UnicodeSegmentation;

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
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        if let Some(width) = self.width {
            if width != area.width {
                panic!("width mismatch");
            }
        } else {
            panic!("unknown width");
        }

        let unicode = UnicodeSegmentation::graphemes(self.content.as_str(), true);

        let cursor_at_end = self.cursor == unicode.clone().count();
        let mut spans = vec![Span::styled(
            unicode
                .clone()
                .skip(self.scroll)
                .take(self.cursor - self.scroll)
                .collect::<String>(),
            self.text_style,
        )];

        if cursor_at_end {
            spans.push(Span::styled(String::from(' '), self.cursor_style));
        } else {
            spans.push(Span::styled(
                unicode
                    .clone()
                    .skip(self.cursor)
                    .take(1)
                    .collect::<String>(),
                self.cursor_style,
            ));
            spans.push(Span::styled(
                unicode.clone().skip(self.cursor + 1).collect::<String>(),
                self.text_style,
            ));
        }

        let paragraph = Paragraph::new(Line::from(spans)).style(self.style);
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
        self.content = format!(
            "{}{}{}",
            UnicodeSegmentation::graphemes(self.content.as_str(), true)
                .take(index)
                .collect::<String>(),
            c,
            UnicodeSegmentation::graphemes(self.content.as_str(), true)
                .skip(index)
                .collect::<String>()
        );
        self.cursor += 1;
        self.update()?;
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<(), TextFieldError> {
        if self.cursor == 0 {
            return Ok(());
        }
        let s = self.content.clone();
        let mut s = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<_>>();
        s.remove(index - 1);
        self.content = s.into_iter().collect();
        self.cursor -= 1;
        self.update()?;
        Ok(())
    }

    pub fn push(&mut self, c: char) -> Result<(), TextFieldError> {
        self.insert(self.cursor, c)
    }

    pub fn pop(&mut self) -> Result<(), TextFieldError> {
        self.remove(self.cursor)
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

        let len = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();

        if self.cursor > len {
            self.cursor = len;
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
