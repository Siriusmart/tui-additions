use std::io::Stdout;
use crossterm::event::KeyEvent;
use dyn_clone::DynClone;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use super::{FrameworkClean, ItemInfo};

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
    fn select(&mut self, framework: &mut FrameworkClean) -> bool {
        true
    }

    /// Deselect the item, if `Ok(())` is return means deselect is successful, or else it failed
    fn deselect(&mut self, framework: &mut FrameworkClean) -> bool {
        true
    }
    fn render(
        &mut self,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        framework: &mut FrameworkClean,
        area: Rect,
        popup_render: bool,
        info: ItemInfo,
    ) {
    }

    /// Runs when `Framework.load_item()` is called
    fn load_item(&mut self, framework: &mut FrameworkClean, info: ItemInfo) {}

    /// Handles key event
    fn key_event(&mut self, framework: &mut FrameworkClean, key: KeyEvent, info: ItemInfo) {}
}

impl Clone for Box<dyn FrameworkItem> {
    fn clone(&self) -> Self {
        *dyn_clone::clone_box(&*self)
    }
}
