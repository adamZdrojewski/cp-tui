use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, List, Widget}};

pub struct WaitTimeList;

impl Widget for WaitTimeList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        List::new(["one", "two", "three"])
            .block(Block::bordered().title("My List"))
            .render(area, buf);
    }
}
