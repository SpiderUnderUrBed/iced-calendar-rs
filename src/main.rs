use iced::{widget::{button, Button, Column, Row, Text}, Application, Element, Settings, Align};
use iced_grid::{Grid, RowData, CellMessage, GridMessage, style};
use std::process::Command;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked(usize, usize), // Track which button was clicked
    AddRow,
}

pub struct MyApp {
    grid: Grid::grid<Message, MyTheme>,
    add_row_button: button::State, // Use button::State directly
}

#[derive(Clone)]
pub struct MyTheme;

impl style::Catalog for MyTheme {
    type Style = ();

    fn TARGET(&self, _style: &Self::Style) -> iced::widget::container::Style {
        // Customize the container style for MyTheme
        iced::widget::container::Style {
            // Example: Set the container's background color
            background: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
            ..Default::default()
        }
    }
}

impl iced::Application for MyApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut grid = Grid::<Message, MyTheme>::Grid(
            "Header".into(),
            "Body".into(),
            vec![], // Initially empty rows
            |offset| Message::ButtonClicked(0, 0), // Dummy sync handler
        );

        // Add some initial rows to the grid
        let mut row = RowData::default();
        row.push_text("Row 1, Cell 1".into());
        row.push_button("Click Me".into(), CellMessage::Clicked);
        grid.add_row(row);

        // Add more rows as needed

        (MyApp {
            grid,
            add_row_button: button::State::new(), // Create a new button state
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Grid Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ButtonClicked(row, col) => {
                // Handle button clicks here
                println!("Button at row {}, col {} clicked", row, col);
            }
            Message::AddRow => {
                // Add a new row to the grid
                let mut new_row = RowData::default();
                new_row.push_text("New Row, Cell 1".into());
                new_row.push_button("New Button".into(), CellMessage::Clicked);
                self.grid.add_row(new_row);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let grid_view = self.grid.view(); // Render the grid

        let add_row_button = Button::new(&mut self.add_row_button, Text::new("Add Row"))
            .on_press(Message::AddRow);

        Column::new()
            .spacing(20)
            .align_items(Align::Center) // This import is fixed here
            .push(add_row_button)
            .push(grid_view)
            .into()
    }
}

fn main() {
    MyApp::run(Settings {
        ..Default::default()
    })
}


// use iced::{Background, Color, widget::{Button, Column, Container, Row, Text, TextInput}, widget::container::{Appearance, StyleSheet}};
// use iced::{Element, Length, Sandbox, Settings};

// pub fn main() -> iced::Result {
//     MyApp::run(Settings::default())
// }

// struct MyApp {
//     rows: Vec<Vec<String>>, // Each row contains a list of cell contents
//     row_input: String,
//     cell_input: String,
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     AddRow,
//     AddCell,
//     UpdateRowInput(String),
//     UpdateCellInput(String),
// }

// impl Sandbox for MyApp {
//     type Message = Message;

//     fn new() -> Self {
//         Self {
//             rows: vec![vec!["Hello, Cell!".to_string()]],
//             row_input: String::new(),
//             cell_input: String::new(),
//         }
//     }

//     fn title(&self) -> String {
//         String::from("My Grid Application")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::AddRow => {
//                 self.rows.push(vec![]); // Add an empty row
//             }
//             Message::AddCell => {
//                 if let Ok(row_index) = self.row_input.parse::<usize>() {
//                     if let Some(row) = self.rows.get_mut(row_index) {
//                         row.push(self.cell_input.clone()); // Add the cell content to the specified row
//                         self.cell_input.clear(); // Clear the input
//                     }
//                 }
//             }
//             Message::UpdateRowInput(input) => {
//                 self.row_input = input;
//             }
//             Message::UpdateCellInput(input) => {
//                 self.cell_input = input;
//             }
//         }
//     }

//     fn view(&self) -> Element<Self::Message> {
//         let mut content = Column::new().spacing(20);

//         // Button to add a new row
//         let add_row_button = Button::new(Text::new("Add Row")).on_press(Message::AddRow);
//         content = content.push(add_row_button);

//         // Row for input and adding a cell
//         content = content.push(
//             Row::new()
//                 .spacing(10)
//                 .push(
//                     TextInput::new("Enter row number", &self.row_input)
//                         .padding(5)
//                         .size(20)
//                         .on_input(Message::UpdateRowInput),
//                 )
//                 .push(
//                     TextInput::new("Enter cell content", &self.cell_input)
//                         .padding(5)
//                         .size(20)
//                         .on_input(Message::UpdateCellInput),
//                 )
//                 .push(Button::new(Text::new("Add Cell")).on_press(Message::AddCell)),
//         );

//         // Display the grid
//         for (row_index, row) in self.rows.iter().enumerate() {
//             let mut row_view = Row::new()
//                 .spacing(10)
//                 .push(Container::new(Text::new(format!("Row {}:", row_index)))
//                     .padding(5)
//                     .width(Length::Shrink)
//                     .center_y());

//             // Create a container for each cell
//             for cell_content in row.iter() {
//                 let cell = Container::new(Text::new(cell_content.clone()))
//                     .padding(10)
//                     .width(Length::Shrink)
//                     .center_y()
//                     .style(|_theme: &_| CellStyle.appearance(&()));// Use a closure to apply the style
//                 row_view = row_view.push(cell);
//             }

//             content = content.push(row_view);
//         }

//         content.into()
//     }
// }

// // Custom style for the cell container
// pub struct CellStyle;

// impl StyleSheet for CellStyle {
//     type Style = (); // The style type for our container (empty in this case)

//     fn appearance(&self, _: &Self::Style) -> Appearance {
//         Appearance {
//             background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))), // Light gray background
//             ..Default::default() // Use default values for other properties
//         }
//     }
// }
