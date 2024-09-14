use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::BorderType,
    Frame, Terminal,
};
use std::{error::Error, io::stdout};
use tui_additions::widgets::TextList;

fn main() -> Result<(), Box<dyn Error>> {
    // enable raw mode for the terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the app until the function ends
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    // generate dummy items for the text list
    let items = (1..101)
        .map(|item| format!("{}. Press Q to exit", item))
        .collect::<Vec<_>>();

    // create the textlist with custom styles
    let mut textlist = TextList::default()
        .style(Style::default().fg(Color::Green).bg(Color::DarkGray))
        .cursor_style(Style::default().fg(Color::Red))
        .selected_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded)
        .items(&items)?;

    // put into an event loop
    loop {
        terminal.draw(|frame| {
            ui(frame, &mut textlist);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                // bit 1 is the shift key modifier, so shift up arrow will go to the first item
                KeyCode::Up if key.modifiers.bits() == 1 => textlist.first()?,
                KeyCode::Down if key.modifiers.bits() == 1 => textlist.last()?,
                KeyCode::Up => textlist.up()?,
                KeyCode::Down => textlist.down()?,
                KeyCode::PageUp => textlist.pageup()?,
                KeyCode::PageDown => textlist.pagedown()?,
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame, list: &mut TextList) {
    // set the height first
    list.set_height(frame.area().height);
    // then render it
    frame.render_widget(list.clone(), frame.area());
}
