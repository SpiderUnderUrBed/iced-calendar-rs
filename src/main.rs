use iced::{Sandbox, Settings};
use iced::widget::{Button, Column, Text}; // Corrected imports
use iced_grid::{CellMessage, Grid, GridMessage, CellConfig};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    grid: Grid,
}

impl Sandbox for MyApp {
    type Message = GridMessage;

    fn new() -> Self {
        let mut grid = Grid::new();
        grid.add_row();
        grid.get_row(0).push(CellConfig::Text("Hello, Cell!".to_string()));

        Self { grid }
    }

    fn title(&self) -> String {
        String::from("My Grid Application")
    }

    fn update(&mut self, message: GridMessage) {
        match message {
            GridMessage::AddCell(row_index) => {
                self.grid
                    .get_row(row_index)
                    .push(CellConfig::Text("New cell".to_string()));
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Edit) => {
                if let Some(cell) = self.grid.get_cell(row_index, cell_index) {
                    cell.edit(CellConfig::Text("Edited!".to_string()));
                }
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Remove) => {
                if let Some(cell) = self.grid.get_cell(row_index, cell_index) {
                    cell.remove();
                }
            }
            GridMessage::Cell(row_index, cell_index, CellMessage::Clicked) => {
                todo!("Handle cell click here.");
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        // Button to add a new row
        let add_row_button = Button::new(Text::new("Add Row"))
            .on_press(GridMessage::AddCell(self.grid.row_count()));

        Column::new()
            .push(add_row_button)
            .push(self.grid.view())
            .into()
    }
}

