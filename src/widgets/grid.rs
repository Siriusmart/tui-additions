use std::{error::Error, fmt::Display};

use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    symbols::line::{self, Set, CROSS},
    widgets::{BorderType, Widget},
};

#[derive(Clone)]
pub struct Grid {
    pub widths: Vec<Constraint>,
    pub heights: Vec<Constraint>,
    pub border_type: BorderType,
    pub border_style: Style,
}

impl Grid {
    pub fn new(widths: Vec<Constraint>, heights: Vec<Constraint>) -> Result<Self, GridError> {
        if widths.is_empty() || heights.is_empty() {
            return Err(GridError::ZeroLength);
        }
        Ok(Self {
            widths,
            heights,
            border_type: BorderType::Plain,
            border_style: Style::default(),
        })
    }
}

impl Grid {
    pub fn chunks(&self, area: Rect) -> Result<Vec<Vec<Rect>>, GridError> {
        let widths = self.widths(area.width)?;
        let heights = self.heights(area.height)?;

        let xs = {
            let mut xs = Self::lines(area.x, &widths);
            xs.truncate(self.widths.len());
            xs.iter_mut().for_each(|item| *item += 1);
            xs
        };
        let ys = {
            let mut ys = Self::lines(area.y, &heights);
            ys.truncate(self.heights.len());
            ys.iter_mut().for_each(|item| *item += 1);
            ys
        };

        Ok(ys
            .iter()
            .zip(heights.iter())
            .map(|(y, height)| {
                let row = xs
                    .iter()
                    .zip(widths.iter())
                    .map(|(x, width)| Rect::new(*x, *y, *width, *height - 1))
                    .collect::<Vec<_>>();
                row
            })
            .collect::<Vec<_>>())
    }

    pub fn lines(mut position: u16, lengths: &[u16]) -> Vec<u16> {
        let mut lines = Vec::new();
        lengths.iter().for_each(|lengths| {
            lines.push(position);
            position += 1 + *lengths;
        });
        // position -= 1;
        lines.push(position);

        lines
    }

    pub fn heights(&self, height: u16) -> Result<Vec<u16>, GridError> {
        Self::lengths(&self.heights, height)
    }

    pub fn widths(&self, width: u16) -> Result<Vec<u16>, GridError> {
        Self::lengths(&self.widths, width - 1)
    }

    pub fn lengths(constraints: &[Constraint], mut length: u16) -> Result<Vec<u16>, GridError> {
        if length < constraints.len() as u16 + 1 {
            return Err(GridError::NotEnoughLength);
        }

        length -= constraints.len() as u16;

        let mut lengths = constraints
            .iter()
            .map(|constraint| constraint.apply(length))
            .collect::<Vec<_>>();
        let sum: u16 = lengths.iter().sum();

        if sum < length {
            *lengths.last_mut().unwrap() += length - sum;
        }
        // .collect::<Vec<_>>();

        Ok(lengths)
    }
}

impl Grid {
    pub fn from_pos<'a>(
        x: &'a u16,
        y: &'a u16,
        left: &'a u16,
        right: &'a u16,
        top: &'a u16,
        bottom: &'a u16,
        set: &'a Set,
    ) -> &'a str {
        let is_top = y == top;
        let is_bottom = y == bottom;
        let is_left = x == left;
        let is_right = x == right;

        if is_top {
            if is_left {
                return set.top_left;
            }

            if is_right {
                return set.top_right;
            }

            return set.horizontal_down;
        }

        if is_bottom {
            if is_left {
                return set.bottom_left;
            }

            if is_right {
                return set.bottom_right;
            }

            return set.horizontal_up;
        }

        if is_left {
            return set.vertical_right;
        }

        if is_right {
            return set.vertical_left;
        }

        CROSS
    }
}

impl Grid {
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.set_border_type(border_type);
        self
    }

    pub fn set_border_type(&mut self, border_type: BorderType) {
        self.border_type = border_type;
    }

    pub fn border_style(mut self, border_style: Style) -> Self {
        self.set_border_style(border_style);
        self
    }

    pub fn set_border_style(&mut self, border_style: Style) {
        self.border_style = border_style;
    }
}

impl Widget for Grid {
    fn render(self, mut area: Rect, buf: &mut ratatui::buffer::Buffer) {
        area.height -= 1;

        let widths = self.widths(area.width).unwrap();
        let heights = self.heights(area.height).unwrap();
        let vertical_lines = Self::lines(area.x, &widths);
        let horizontal_lines = Self::lines(area.y, &heights);

        let top = horizontal_lines.first().unwrap();
        let bottom = horizontal_lines.last().unwrap();
        let left = vertical_lines.first().unwrap();
        let right = vertical_lines.last().unwrap();

        let set = match self.border_type {
            BorderType::Plain => line::NORMAL,
            BorderType::Thick => line::THICK,
            BorderType::Double => line::DOUBLE,
            BorderType::Rounded => line::ROUNDED,
            _ => panic!("no such line type"),
        };

        // vertical lines
        for x in vertical_lines.iter() {
            for y in *top..*bottom + 1 {
                if !horizontal_lines.contains(&y) {
                    buf.set_string(*x, y, set.vertical, self.border_style);
                }
            }
        }

        // horizontal lines
        for y in horizontal_lines.iter() {
            for x in *left..*right + 1 {
                if vertical_lines.contains(&x) {
                    buf.set_string(
                        x,
                        *y,
                        Self::from_pos(&x, y, left, right, top, bottom, &set),
                        self.border_style,
                    );
                } else {
                    buf.set_string(x, *y, set.horizontal, self.border_style);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    NotEnoughLength,
    ZeroLength,
}

impl Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for GridError {}
