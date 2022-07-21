use std::{error::Error, io::Stdout};

use crossterm::event::KeyEvent;
use dyn_clone::DynClone;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use super::FrameworkClean;

/// Trait every item on `State` should implment
///
/// Only include functions if you want to change
#[allow(unused)]
pub trait FrameworkItem: DynClone {
    /// If the item is selectable (if not the cursor will not be able to hover or select that item)
    fn selectable(&self) -> bool {
        true
    }

    /// Select the item, if `Ok(())` is return means select is successful, or else it failed
    fn select(&mut self, framework: &FrameworkClean) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Deselect the item, if `Ok(())` is return means deselect is successful, or else it failed
    fn deselect(&mut self, framework: &FrameworkClean) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn render(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        framework: &FrameworkClean,
        area: Rect,
        selected: bool,
        hover: bool,
        popup_render: bool,
    ) {
    }

    /// Runs when `Framework.load_item()` is called
    fn load_item(&mut self, framework: &FrameworkClean) {}

    /// Handles key event
    fn key_event(&mut self, framework: FrameworkClean, key: KeyEvent) {}
}
