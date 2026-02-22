use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Borders, List, Paragraph, Widget}, DefaultTerminal, Frame};

pub struct App {
    pub exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        // Main loop
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }

    fn render_title(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Cedar Point")
            .bold()
            .block(Block::default()
                .borders(Borders::ALL))
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) where
        Self: Sized {
        let horizontal_layout = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);
        let [left_side, right_side] = horizontal_layout.areas(area);

        App::render_title(left_side, buf);

        List::new(["one", "two", "three"])
            .block(Block::bordered().title("My List"))
            .render(right_side, buf);
    }
}

pub enum Event {
    Input(crossterm::event::KeyEvent)
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}
