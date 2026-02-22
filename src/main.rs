use std::{io, sync::mpsc, thread};

use cp_tui::{handle_input_events, App, Event};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false
    };

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result
}
