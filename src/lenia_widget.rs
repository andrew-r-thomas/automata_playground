// use iced::{
//     mouse,
//     widget::shader::{
//         wgpu::{self, include_wgsl, util::DeviceExt, Buffer, RenderPipeline},
//         Primitive, Program,
//     },
//     Size,
// };

// #[derive(Debug)]
// pub struct LeniaPrimitive {}

// impl Primitive for LeniaPrimitive {
//     fn prepare(
//         &self,
//         format: iced::widget::shader::wgpu::TextureFormat,
//         device: &iced::widget::shader::wgpu::Device,
//         queue: &iced::widget::shader::wgpu::Queue,
//         bounds: iced::Rectangle,
//         target_size: iced::Size<u32>,
//         scale_factor: f32,
//         storage: &mut iced::widget::shader::Storage,
//     ) {
//         if !storage.has::<LeniaPipeline>() {
//             storage.store(LeniaPipeline::new(device, queue, format, target_size));
//         }
//     }

//     fn render(
//         &self,
//         storage: &iced::widget::shader::Storage,
//         target: &iced::widget::shader::wgpu::TextureView,
//         target_size: iced::Size<u32>,
//         viewport: iced::Rectangle<u32>,
//         encoder: &mut iced::widget::shader::wgpu::CommandEncoder,
//     ) {
//         let pipeline = storage.get::<LeniaPipeline>().unwrap();

//         pipeline.render(encoder, target);
//     }
// }

// pub struct LeniaProgram {}

// impl<Message> Program<Message> for LeniaProgram {
//     type State = ();
//     type Primitive = LeniaPrimitive;

//     fn draw(
//         &self,
//         state: &Self::State,
//         cursor: mouse::Cursor,
//         bounds: iced::Rectangle,
//     ) -> Self::Primitive {
//         Self::Primitive {}
//     }
// }

// struct LeniaPipeline {
//     pipeline: RenderPipeline,
//     vertex_buffer: Buffer,
// }

// impl LeniaPipeline {
//     pub fn new(
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         format: wgpu::TextureFormat,
//         target_size: Size<u32>,
//     ) -> Self {
//         let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Vertex Buffer"),
//             contents: bytemuck::cast_slice(VERTICES),
//             usage: wgpu::BufferUsages::VERTEX,
//         });

//         let shader = device.create_shader_module(include_wgsl!("shaders/triangle_shader.wgsl"));
//         let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("pipeline layout"),
//             bind_group_layouts: &[],
//             push_constant_ranges: &[],
//         });

//         let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("triangle pipeline"),
//             layout: Some(&layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: "vs_main",
//                 buffers: &[Vertex::desc()],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &shader,
//                 entry_point: "fs_main",
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format,
//                     blend: Some(wgpu::BlendState::REPLACE),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//             }),
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: Some(wgpu::Face::Back),
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 unclipped_depth: false,
//                 conservative: false,
//             },
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: 1,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview: None,
//         });

//         Self {
//             pipeline,
//             vertex_buffer,
//         }
//     }

//     pub fn render(&self, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
//         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//             label: Some("Render Pass"),
//             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                 view: &target,
//                 resolve_target: None,
//                 ops: wgpu::Operations {
//                     load: wgpu::LoadOp::Clear(wgpu::Color {
//                         r: 0.1,
//                         g: 0.2,
//                         b: 0.3,
//                         a: 1.0,
//                     }),
//                     store: wgpu::StoreOp::Store,
//                 },
//             })],
//             depth_stencil_attachment: None,
//             occlusion_query_set: None,
//             timestamp_writes: None,
//         });
//         render_pass.set_pipeline(&self.pipeline);
//         render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
//         render_pass.draw(0..3, 0..1);
//     }
// }

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//     position: [f32; 3],
//     color: [f32; 3],
// }

// const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [0.0, 0.5, 0.0],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.0, 1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.0, 0.0, 1.0],
//     },
// ];

// impl Vertex {
//     const ATTRIBS: [wgpu::VertexAttribute; 2] =
//         wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

//     fn desc() -> wgpu::VertexBufferLayout<'static> {
//         use std::mem;

//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &Self::ATTRIBS,
//         }
//     }
// }
