use iced::{Application, executor, Command, widget::{row, container}, widget::{column as col, button}, theme::Text, Settings};

#[derive(Debug, Clone, Copy)]
enum Messages {
    Button1Clicked,
}

struct ExampleApp {}

impl Application for ExampleApp {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (ExampleApp {
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("example app")
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        col!(
            container(
                button("Click me!").on_press(Messages::Button1Clicked)
            ).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center)
        ).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::Button1Clicked => {
                println!("You clicked me!");
            }
        }
        
        Command::none()
    }
}

fn main() {
    let (app, command) = ExampleApp::new(());
    
    ExampleApp::run(Settings::default());
}
