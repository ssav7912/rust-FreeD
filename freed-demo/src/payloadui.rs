use std::vec;

use freed::freed::{PositionPollPayload, PollPayload};
use tui::{backend::Backend, layout::{Rect, Layout, Direction, Constraint}, Frame, widgets::{Table, Paragraph, Block, Cell, Row, Borders}};

use crate::even_columns;
pub trait StructUI {    

    fn to_array_of_strings(self) -> Vec<[String; 3]>;

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend;
 
}

impl StructUI for PollPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![["Command".to_string(), format!("{:?}", self.command), "".to_string()]]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        let innerlayout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(10), Constraint::Percentage(90)]).split(area);

        let struct_title = Paragraph::new("Poll Payload").block(Block::default().borders(Borders::NONE));
        f.render_widget(struct_title, innerlayout[0]);

        let header_cells = ["Field", "Value", "Unit"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = fields.iter().map(|item| {
            let cells = item.iter().map(|c| Cell::from(c.as_ref()));
            Row::new(cells).height(1)
        });        

        
        let binding = even_columns(3);
        let table = Table::new(rows).header(header).block(Block::default().borders(Borders::ALL)).
        widths(binding.as_ref());
        f.render_widget(table, innerlayout[1]);
    }
}

impl StructUI for PositionPollPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![["pitch".to_string(), self.pitch.to_string(), "Millimetre".to_string()],
         ["yaw".to_string(), self.yaw.to_string(), "Millimetre".to_string()], 
         ["roll".to_string(), self.roll.to_string(), "Millimetre".to_string()],
         ["pos_z".to_string(), self.pos_z.to_string(), "Millimetre".to_string()],
         ["pos_y".to_string(), self.pos_y.to_string(), "Millimetre".to_string()],
         ["pos_x".to_string(), self.pos_x.to_string(), "Millimetre".to_string()],
         ["zoom".to_string(), self.zoom.to_string(), "Millimetre".to_string()],
         ["focus".to_string(), self.focus.to_string(), "".to_string()]
         ]
    }


    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        let innerlayout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(10), Constraint::Percentage(90)]).split(area);

        let struct_title = Paragraph::new("Position Poll Payload").block(Block::default().borders(Borders::NONE));
        f.render_widget(struct_title, innerlayout[0]);

        let header_cells = ["Field", "Value", "Unit"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = fields.iter().map(|item| {
            let cells = item.iter().map(|c| Cell::from(c.as_ref()));
            Row::new(cells).height(1)
        });        

        
        let binding = even_columns(3);
        let table = Table::new(rows).header(header).block(Block::default().borders(Borders::ALL)).
        widths(binding.as_ref());
        f.render_widget(table, innerlayout[1]);
    }
}