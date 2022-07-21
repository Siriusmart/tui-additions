//! # TUI Additions
//!
//! Additions to the TUI crate
//!
//! ![](https://raw.githubusercontent.com/Siriusmart/tui-additions/master/assets/framework.gif)

/// Additional widgets (structs that impl `tui::widget::Widget`)
#[cfg(feature = "widgets")]
pub mod widgets;

/// A declarative TUI framework
#[cfg(feature = "framework")]
pub mod framework;
