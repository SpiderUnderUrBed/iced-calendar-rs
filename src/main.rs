use iced::advanced::graphics::core::Element;
use iced::advanced::graphics::futures::backend::default;
use iced::advanced::graphics::text::cosmic_text::Wrap;
use iced::application::Title;
use iced::daemon::{Appearance, DefaultStyle};
use iced::widget::container;
use iced::{Application, Color, Settings, Size, Subscription, Theme};
use iced_grid::{CellMessage, Grid, GridMessage, RowData};
use iced_grid::style::wrapper::{Style, Wrapper};

#[derive(Debug, Clone)]
enum Message {
    Ui(UiMessage),
    Grid(iced_grid::GridMessage),
    //Sync,
}

#[derive(Debug, Clone)]
enum UiMessage {
    AddRow,
    AddCell(usize), 
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

pub struct MyApp 
where 
    Message: Clone
{
    grid: Grid<Message, MyTheme>,
}

use iced::{Background};

#[derive(Debug, Clone)]
pub struct MyStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub padding: f32,
}


impl Default for MyApp {
    fn default() -> Self {
        let rows = vec![];

        
        let mut grid: Grid<Message, MyTheme> = Grid::new(
            rows,
            container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            },
            |_offset: iced::widget::scrollable::AbsoluteOffset| UiMessage::Sync.into(),
            400.0,
            400.0,
            Size::new(100.0, 100.0),
            MyTheme::Main
            
        );

        
        let mut row = RowData::default();
        row.push_text("Row 1, Cell 1".into());
        row.push_button("Add Row".into(), CellMessage::Clicked);
        row.push_button("Add Cell".into(), CellMessage::Clicked);
        row.push_container(container("New Cell").center(100));
        grid.add_row(row);
        let mut row2 = RowData::default();
        grid.add_row(row2);
        
        grid.add_cells_to_all_rows(5);
        grid.style(
            container::Style {
                background: Some(Background::Color(Color::BLACK)),
                ..Default::default()
            }
        );
        
        


        MyApp { grid }
    }
}














#[derive(Clone, Default)]
pub enum MyTheme{   
    #[default]
    Main,
    
}

impl DefaultStyle for MyTheme {
    fn default_style(&self) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE
        }
    }
}

impl iced::widget::container::Catalog for MyTheme {
    type Class<'a> = iced::widget::container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| iced::widget::container::Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::container::Style {
        class(self)
    }
}
impl iced::widget::text::Catalog for MyTheme {
    type Class<'a> = iced::widget::text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| iced::widget::text::Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::text::Style {
        class(self)
    }
}

impl iced_grid::style::Catalog for MyTheme {
    type Style = <Theme as iced_grid::style::Catalog>::Style;
    type Themes = iced::Theme;

    fn body(&self, _style: &Self::Style) -> iced::widget::container::Style {
            match self {
                &MyTheme::Main => container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
                ..Default::default()
            }
        }
    }

    fn cell(&self, _row: usize, _col: usize) -> iced::widget::container::Style {
        match self {
            &MyTheme::Main => container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.9))),
                ..Default::default()
            }
        }
    }
    
    fn resolve_theme(&self) -> Self::Themes {
        iced::Theme::Dark
        
    }
}

impl MyApp {
    fn view(&mut self) -> iced::Element<'_, Message, MyTheme>{
        
        let grid: Grid<GridMessage, MyTheme> = Grid::new(
            self.grid.rows_mut_iter().map(|row| RowData {
                cells: std::mem::take(&mut row.cells), // Move `cells` instead of cloning
            }),
            container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            },
            |_offset: iced::widget::scrollable::AbsoluteOffset| GridMessage::Sync,
            400.0,
            400.0,
            Size::new(100.0, 100.0),
            MyTheme::Main
        );

         iced::Element::new(Wrapper {
             content: Box::new(&self.grid),
             target: Style,
             theme: self.grid.theme.clone(),
             style: self.grid.style,
         })
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
                        let cell_count = row.cells.len() - 2; 
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
                    
                    if col == 1 {
                        
                        self.update(Message::Ui(UiMessage::AddRow));
                    } else if col == 2 {
                        
                        self.update(Message::Ui(UiMessage::AddCell(row)));
                    }
                }
                _ => {
                    
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
    iced::run("main", MyApp::update, |arg0: &MyApp| MyApp::view(&mut *arg0))
}
