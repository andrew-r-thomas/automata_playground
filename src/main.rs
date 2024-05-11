// pub mod lenia_widget;

// use iced_baseview::core::{Element, Length};
// use iced_baseview::Application;
// use iced_baseview::Settings;
// use lenia_widget::LeniaProgram;

// struct Playground {
//     lenia: LeniaProgram,
// }

// impl Application for Playground {
//     type Message = ();

//     fn new() -> Self {
//         Self {
//             lenia: LeniaProgram {},
//         }
//     }

//     fn title(&self) -> String {
//         String::from("A cool application")
//     }

//     fn update(&mut self, _message: Self::Message) {
//         // This application has no interactions
//     }

//     fn view(&self) -> Element<Self::Message> {
//         shader(&self.lenia)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .into()
//     }
// }

pub mod lenia_widget;

// fn main() {
//     Playground::run(Settings::default()).unwrap()
// }
fn main() {}
