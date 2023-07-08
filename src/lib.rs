//! # TUI Additions
//!
//! Additions to the TUI crate
//!
//! ![](https://raw.githubusercontent.com/Siriusmart/tui-additions/master/assets/framework.gif)

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
/// Additional widgets (structs that impl `ratatui::widget::Widget`)
#[cfg(feature = "widgets")]
pub mod widgets;

/// A declarative TUI framework
#[cfg(feature = "framework")]
pub mod framework;

#[cfg(test)]
mod tests;
