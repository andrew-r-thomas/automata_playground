pub mod lenia_widget;

use iced::widget::shader;
use iced::Element;
use iced::Length;
use iced::Settings;
use lenia_widget::LeniaProgram;

use iced::Sandbox;

struct Playground {
    lenia: LeniaProgram,
}

impl Sandbox for Playground {
    type Message = ();

    fn new() -> Self {
        Self {
            lenia: LeniaProgram {},
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        shader(&self.lenia)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() {
    Playground::run(Settings::default()).unwrap()
}
