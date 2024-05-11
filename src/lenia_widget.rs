use iced::{
    mouse,
    widget::shader::{Primitive, Program},
};

#[derive(Debug)]
struct LeniaPrimitive {}

impl Primitive for LeniaPrimitive {
    fn prepare(
        &self,
        format: iced::widget::shader::wgpu::TextureFormat,
        device: &iced::widget::shader::wgpu::Device,
        queue: &iced::widget::shader::wgpu::Queue,
        bounds: iced::Rectangle,
        target_size: iced::Size<u32>,
        scale_factor: f32,
        storage: &mut iced::widget::shader::Storage,
    ) {
        todo!()
    }

    fn render(
        &self,
        storage: &iced::widget::shader::Storage,
        target: &iced::widget::shader::wgpu::TextureView,
        target_size: iced::Size<u32>,
        viewport: iced::Rectangle<u32>,
        encoder: &mut iced::widget::shader::wgpu::CommandEncoder,
    ) {
        todo!()
    }
}

struct LeniaProgram {}

impl<Message> Program<Message> for LeniaProgram {
    type State = ();
    type Primitive = LeniaPrimitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        todo!();
    }
}
