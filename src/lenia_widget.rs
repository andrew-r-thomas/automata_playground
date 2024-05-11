use iced::{
    mouse,
    widget::shader::{
        wgpu::{self, include_wgsl, util::DeviceExt},
        Primitive, Program,
    },
    Size,
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

struct LeniaPipeline {}

impl LeniaPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        target_size: Size<u32>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let shader = device.create_shader_module(include_wgsl!("shaders/triangle_shader.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
        });

        Self {}
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
