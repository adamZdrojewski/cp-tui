use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, Paragraph, Widget}};

pub struct Header;

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Cedar Point")
            .bold()
            .block(Block::bordered())
            .render(area, buf);
    }
}
