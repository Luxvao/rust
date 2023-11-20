use std::{collections::HashMap, cell::{RefCell}};
use iced::{widget::{button, text, scrollable, Row, Scrollable, container, text_input}, Settings, Application, Theme, Command, Length};
use iced::widget::{column as col, row};
use serde::{Serialize, Deserialize};

fn main() {
    FileExplorer::run(Settings::with_flags(())).unwrap();   
    std::io::stdin();
}

#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    cache: HashMap<String, String>,
}

enum States {
    DefaultState,
    NewFolderState,
    AddSomethingToQuickAccessMenu,
}

struct FileExplorer<'a> {
    cache: RefCell<Cache>,
    saved_locations: RefCell<Cache>,
    state: States,
    row1: Row<'a, Messages>,
    input_text_new_item_name: String,
}

#[derive(Clone, Debug)]
enum Messages {
    OpenFile(String),
    SearchButtonPressed,
    NewFolderButtonPressed,
    DefaultStateButtonClicked,
    AddItemsQuickAccessMenu,
}

impl<'a> Application for FileExplorer<'a> {
    type Message = Messages;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (FileExplorer { cache: RefCell::new(Cache { cache: HashMap::new() }), input_text_new_item_name: String::new(), row1: row!(button("Add New").on_press(Messages::AddItemsQuickAccessMenu)), state: States::DefaultState, saved_locations: RefCell::new(Cache { cache: HashMap::new() })}, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("File Explorer 2.0")
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.state {
            States::DefaultState => {
                row!(
                    scrollable(

                    )
                )
                .into()
            }
            States::NewFolderState => {
                col!(
                    button("Go back!").on_press(Messages::DefaultStateButtonClicked),
                    button("Hi!").on_press(Messages::SearchButtonPressed),
                    text(":D")
                )
                .into()
            }
            States::AddSomethingToQuickAccessMenu => {
                row!(
                    text_input("Input new item", "test")
                )
                .into()
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Messages::SearchButtonPressed => {
                
            }
            Messages::OpenFile(file_path) => {

            }
            Messages::DefaultStateButtonClicked => {
                self.state = States::DefaultState;
            }
            Messages::NewFolderButtonPressed => {
                self.state = States::NewFolderState;
            }
            Messages::AddItemsQuickAccessMenu => {
                self.state = States::AddSomethingToQuickAccessMenu;
            }
        }

        Command::none()
    }
}
