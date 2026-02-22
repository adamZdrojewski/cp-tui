use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::{Line, Span}, widgets::{Block, List, ListItem, Widget}};

pub struct WaitTimeList;

impl Widget for WaitTimeList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = [
            "Maverick",
            "Steel Vengeance",
            "Millennium Force",
            "Valravn",
            "Raptor",
            "GateKeeper",
            "Magnum XL-200",
            "Rougarou",
            "Wild Mouse",
            "Gemini",
            "Iron Dragon",
            "Blue Streak",
            "Corkscrew",
            "Cedar Creek Mine Ride",
            "Top Thrill 2",
            "Siren's Curse"
        ];
        let items: Vec<ListItem> = items.iter().enumerate().map(|(_i, item)| {
            let line = Line::from(vec![
                Span::from(item.to_string()),
                Span::from(" - "),
                Span::from("50 mins").yellow()
            ]);

            ListItem::new(line)
        }).collect();

        List::new(items)
            .block(Block::bordered().title("My List"))
            .render(area, buf);
    }
}
