use super::{FrameworkClean, ItemInfo};
use crossterm::event::KeyEvent;
use dyn_clone::DynClone;
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame};
use std::{any::Any, error::Error, io::Stdout};

/// Trait every item on `State` should implment
///
/// Only include functions if you want to change
#[allow(unused)]
pub trait FrameworkItem: DynClone + Any {
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
    fn load_item(
        &mut self,
        framework: &mut FrameworkClean,
        info: ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Handles key event
    fn key_event(
        &mut self,
        framework: &mut FrameworkClean,
        key: KeyEvent,
        info: ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn mouse_event(
        &mut self,
        framework: &mut FrameworkClean,
        x: u16,
        y: u16,
        absolute_x: u16,
        absolute_y: u16,
    ) -> bool {
        false
    }
}

impl Clone for Box<dyn FrameworkItem> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}
