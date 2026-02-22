mod components;
pub mod events;

use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::Widget, DefaultTerminal, Frame};

use crate::{components::{header::Header, wait_time_list::WaitTimeList}, events::Event};

pub struct App {
    pub exit: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false
        }
    }

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
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) where
        Self: Sized {
        let horizontal_layout = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);
        let [left_side, right_side] = horizontal_layout.areas(area);

        Header.render(left_side, buf);
        WaitTimeList.render(right_side, buf);
    }
}
