use iced::application::Title;
use iced::widget::container;
use iced::{Application, Color, Element, Settings, Subscription, Theme};
use iced_grid::{Grid, RowData, CellMessage};

#[derive(Debug, Clone)]
enum Message {
    Ui(UiMessage),
    Grid(iced_grid::GridMessage),
}

#[derive(Debug, Clone)]
enum UiMessage {
    AddRow,
    AddCell(usize), // usize represents the row to which a cell will be added
    ButtonClicked(usize, usize),
    Sync,
}

impl From<UiMessage> for Message {
    fn from(ui_message: UiMessage) -> Self {
        Message::Ui(ui_message)
    }
}

impl From<iced_grid::GridMessage> for Message {
    fn from(grid_message: iced_grid::GridMessage) -> Self {
        Message::Grid(grid_message)
    }
}

pub struct MyApp {
    grid: Grid<Message, MyTheme>,
}

use iced::{Background};

#[derive(Debug, Clone)]
pub struct MyStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub padding: f32,
}

impl Default for MyStyle {
    fn default() -> Self {
        MyStyle {
            background_color: Color::from_rgb(0.2, 0.4, 0.6),
            text_color: Color::WHITE,
            padding: 10.0,
        }
    }
}

impl From<MyStyle> for container::Style {
    fn from(style: MyStyle) -> Self {
        container::Style {
            background: Some(Background::Color(style.background_color)),
            ..Default::default()
        }
    }
}


impl Default for MyApp {
    fn default() -> Self {
        let rows = vec![];

        // Create the grid
        let mut grid = Grid::new(
            rows,
            container::Style {
                background: Some(Background::Color(Color::BLACK)),
                ..Default::default()
            },
            |_offset: iced::widget::scrollable::AbsoluteOffset| UiMessage::Sync.into(),
        );

        // Add an initial row to the grid
        let mut row = RowData::default();
        row.push_text("Row 1, Cell 1".into());
        row.push_button("Add Row".into(), CellMessage::Clicked);
        row.push_button("Add Cell".into(), CellMessage::Clicked);
        grid.add_row(row);
        grid.style(
                container::Style {
                background: Some(Background::Color(Color::BLACK)),
                ..Default::default()
            }
        );
        
        


        MyApp { grid }
    }
}

#[derive(Clone)]
pub struct MyTheme;

impl iced_grid::style::Catalog for MyTheme {
    type Style = container::Style;

    fn style(&self, style: &Self::Style) -> iced::widget::container::Style {
        container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            ..Default::default()
        }
    }    
}

impl MyApp {
    fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        self.grid.view().map(Message::from)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Ui(ui_message) => match ui_message {
                UiMessage::AddRow => {
                    let mut new_row = RowData::default();
                    let row_index = self.grid.row_count();
                    new_row.push_text(format!("Row {}, Cell 1", row_index + 1).into());
                    new_row.push_button("Add Row".into(), CellMessage::Clicked);
                    new_row.push_button("Add Cell".into(), CellMessage::Clicked);
                    self.grid.add_row(new_row);
                }
                UiMessage::AddCell(row_index) => {
                    if let Some(row) = self.grid.get_row_mut(row_index) {
                        let cell_count = row.cells.len() - 2; // Exclude Add Row and Add Cell buttons
                        row.push_text(format!("Row {}, Cell {}", row_index + 1, cell_count + 1).into());
                    }
                }
                UiMessage::ButtonClicked(row, col) => {
                    println!("Button clicked in row {}, column {}", row, col);
                }
                UiMessage::Sync => {
                    println!("Syncing...");
                }
            },
            Message::Grid(grid_message) => match grid_message {
                iced_grid::GridMessage::Cell(row, col, CellMessage::Clicked) => {
                    // Determine action based on the column index
                    if col == 1 {
                        // Add Row button clicked
                        self.update(Message::Ui(UiMessage::AddRow));
                    } else if col == 2 {
                        // Add Cell button clicked
                        self.update(Message::Ui(UiMessage::AddCell(row)));
                    }
                }
                _ => {
                    // Handle other grid messages if necessary
                    println!("Grid message received: {:?}", grid_message);
                }
            },
        }
    }
    

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

fn main() -> iced::Result {
    iced::run("main", MyApp::update, MyApp::view)
}
