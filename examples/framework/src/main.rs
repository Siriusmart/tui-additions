use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    fmt::Display,
    io::{stdout, Stdout},
};
use tui_additions::{
    framework::{
        Framework, FrameworkClean, FrameworkDirection, FrameworkItem, ItemInfo, Row, RowItem, State,
    },
    widgets::TextList,
};
use typemap::Key;

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let state = State(vec![
        Row {
            items: vec![
                RowItem {
                    item: Box::new(TextBox::new("Test", true)),
                    width: Constraint::Length(10),
                },
                RowItem {
                    item: Box::new(TextBox::new("Hello world", true)),
                    width: Constraint::Length(50),
                },
            ],
            centered: true,
            height: Constraint::Length(5),
        },
        Row {
            items: vec![RowItem {
                item: Box::new(List::new()),
                width: Constraint::Length(60),
            }],
            centered: true,
            height: Constraint::Length(10),
        },
        Row {
            items: vec![RowItem {
                item: Box::new(KeyPressDisplay),
                width: Constraint::Length(40),
            }],
            centered: true,
            height: Constraint::Length(3),
        },
    ]);

    let mut app = Framework::new(state);

    app.data
        .state
        .insert::<KeyLastPressed>(KeyLastPressed(None));

    loop {
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        if let Event::Key(key) = event::read()? {
            app.data
                .state
                .insert::<KeyLastPressed>(KeyLastPressed(Some(key.code)));

            if key.code == KeyCode::Esc {
                _ = app.deselect();
            }

            if app.cursor.selected(&app.selectables).is_some() {
                app.key_input(key).unwrap();
            } else {
                match key.code {
                    KeyCode::Up => _ = app.r#move(FrameworkDirection::Up),
                    KeyCode::Down => _ = app.r#move(FrameworkDirection::Down),
                    KeyCode::Left => _ = app.r#move(FrameworkDirection::Left),
                    KeyCode::Right => _ = app.r#move(FrameworkDirection::Right),
                    KeyCode::Enter => _ = app.select(),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TextBox {
    pub text: String,
    pub selectable: bool,
}

impl TextBox {
    pub fn new(text: &str, selectable: bool) -> Self {
        Self {
            text: text.to_string(),
            selectable,
        }
    }
}

impl FrameworkItem for TextBox {
    fn render(
        &mut self,
        frame: &mut Frame,
        _framework: &mut FrameworkClean,
        area: Rect,
        _popup_render: bool,
        info: ItemInfo,
    ) {
        let border_color = if info.hover {
            Color::Red
        } else if info.selected {
            Color::LightBlue
        } else {
            Color::Reset
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        let paragraph = Paragraph::new(self.text.clone()).block(block);

        frame.render_widget(paragraph, area);
    }

    fn selectable(&self) -> bool {
        self.selectable
    }
}

#[derive(Clone)]
pub struct List {
    pub textlist: TextList,
}

impl List {
    fn new() -> Self {
        let items = (1..100).collect::<Vec<_>>();

        Self {
            textlist: TextList::default().items(&items).unwrap(),
        }
    }
}

impl FrameworkItem for List {
    fn render(
        &mut self,
        frame: &mut Frame,
        _framework: &mut FrameworkClean,
        area: Rect,
        _popup_render: bool,
        info: ItemInfo,
    ) {
        let border_color = if info.hover {
            Color::Red
        } else if info.selected {
            Color::LightBlue
        } else {
            Color::Reset
        };
        let selected_color = if info.selected {
            Color::LightYellow
        } else {
            Color::Reset
        };
        let cursor_color = if info.selected {
            Color::LightRed
        } else {
            Color::Reset
        };
        let style = if info.selected {
            Color::LightGreen
        } else {
            Color::Reset
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(String::from("Text List"))
            .border_style(Style::default().fg(border_color));
        let inner = block.inner(area);

        frame.render_widget(block, area);

        self.textlist.set_height(inner.height);
        self.textlist
            .set_selected_style(Style::default().fg(selected_color));
        self.textlist
            .set_cursor_style(Style::default().fg(cursor_color));
        self.textlist.set_style(Style::default().fg(style));
        frame.render_widget(self.textlist.clone(), inner);
    }

    fn key_event(
        &mut self,
        _framework: &mut FrameworkClean,
        key: KeyEvent,
        _info: ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        match key.code {
            // bit 1 is the shift key modifier, so shift up arrow will go to the first item
            KeyCode::Up if key.modifiers.bits() == 1 => self.textlist.first().unwrap(),
            KeyCode::Down if key.modifiers.bits() == 1 => self.textlist.last().unwrap(),
            KeyCode::Up => self.textlist.up().unwrap(),
            KeyCode::Down => self.textlist.down().unwrap(),
            KeyCode::PageUp => self.textlist.pageup().unwrap(),
            KeyCode::PageDown => self.textlist.pagedown().unwrap(),
            _ => {}
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct KeyPressDisplay;

impl FrameworkItem for KeyPressDisplay {
    fn render(
        &mut self,
        frame: &mut Frame,
        framework: &mut FrameworkClean,
        area: Rect,
        _popup_render: bool,
        _info: ItemInfo,
    ) {
        let block = Block::default().borders(Borders::ALL).title("Key pressed");
        let paragraph = Paragraph::new(format!(
            "{}",
            framework.data.state.get::<KeyLastPressed>().unwrap()
        ))
        .block(block);

        frame.render_widget(paragraph, area);
    }

    fn selectable(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct KeyLastPressed(Option<KeyCode>);

impl Key for KeyLastPressed {
    type Value = Self;
}

impl Display for KeyLastPressed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(keycode) => f.write_fmt(format_args!("{:?}", keycode)),
            None => f.write_str("No keys pressed"),
        }
    }
}
