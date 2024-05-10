use iced::Element;
use iced::Sandbox;
use iced::Settings;

struct Playground {}

impl Sandbox for Playground {
    type Message = ();

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }
}

fn main() {
    Playground::run(Settings::default()).unwrap()
}
