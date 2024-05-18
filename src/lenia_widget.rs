use iced::{
    mouse,
    widget::shader::{
        wgpu::{
            self,
            core::id::TextureViewId,
            include_wgsl,
            util::{BufferInitDescriptor, DeviceExt},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingResource,
            BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline,
            RenderPipeline, ShaderStages, Texture, TextureDescriptor, TextureFormat, TextureUsages,
            TextureView, TextureViewDescriptor,
        },
        Primitive, Program,
    },
};

const WORKGROUP_SIZE: u32 = 8;
const GAME_SIZE: u32 = 512;

#[derive(Debug)]
pub struct LeniaPrimitive {}

impl Primitive for LeniaPrimitive {
    fn prepare(
        &self,
        format: iced::widget::shader::wgpu::TextureFormat,
        device: &iced::widget::shader::wgpu::Device,
        queue: &iced::widget::shader::wgpu::Queue,
        _bounds: iced::Rectangle,
        _target_size: iced::Size<u32>,
        _scale_factor: f32,
        storage: &mut iced::widget::shader::Storage,
    ) {
        if !storage.has::<LeniaPipeline>() {
            storage.store(LeniaPipeline::new(device, queue, format));
        }

        let lenia = storage.get_mut::<LeniaPipeline>().unwrap();
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

        lenia.setup_board(&mut encoder);
    }

    fn render(
        &self,
        storage: &iced::widget::shader::Storage,
        target: &iced::widget::shader::wgpu::TextureView,
        _target_size: iced::Size<u32>,
        _viewport: iced::Rectangle<u32>,
        encoder: &mut iced::widget::shader::wgpu::CommandEncoder,
    ) {
        let pipe = storage.get::<LeniaPipeline>().unwrap();
        pipe.render(encoder, target);
    }
}

pub struct LeniaProgram {}

impl<Message> Program<Message> for LeniaProgram {
    type State = ();
    type Primitive = LeniaPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Self::Primitive {}
    }
}

struct LeniaPipeline {
    // init_pipeline: ComputePipeline,
    // update_pipeline: ComputePipeline,
    render_pipeline: RenderPipeline,
    // game_buff_a: Texture,
    // game_buff_b: Texture,
    // bind_group: BindGroup,
}

impl LeniaPipeline {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/lenia.wgsl"));
        // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
        //             ty: wgpu::BindingType::StorageTexture {
        //                 access: wgpu::StorageTextureAccess::WriteOnly,
        //                 format: TextureFormat::R32Float,
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //             },
        //             count: None,
        //         },
        //         BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
        //             ty: wgpu::BindingType::StorageTexture {
        //                 access: wgpu::StorageTextureAccess::WriteOnly,
        //                 format: TextureFormat::R32Float,
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //             },
        //             count: None,
        //         },
        //         BindGroupLayoutEntry {
        //             binding: 2,
        //             visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         },
        //     ],
        // });

        // let game_buff_a = device.create_texture(&TextureDescriptor {
        //     label: Some("game buff a"),
        //     size: wgpu::Extent3d {
        //         width: 512,
        //         height: 512,
        //         depth_or_array_layers: 0,
        //     },
        //     mip_level_count: 0,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: TextureFormat::R32Float,
        //     usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        //     view_formats: &[],
        // });
        // let game_buff_b = device.create_texture(&TextureDescriptor {
        //     label: Some("game buff b"),
        //     size: wgpu::Extent3d {
        //         width: 512,
        //         height: 512,
        //         depth_or_array_layers: 0,
        //     },
        //     mip_level_count: 0,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: TextureFormat::R32Float,
        //     usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        //     view_formats: &[],
        // });

        // let game_view_a = game_buff_a.create_view(&TextureViewDescriptor {
        //     label: Some("game view a"),
        //     format: Some(TextureFormat::R32Float),
        //     base_mip_level: 0,
        //     mip_level_count: Some(1),
        //     ..Default::default()
        // });
        // let game_view_b = game_buff_b.create_view(&TextureViewDescriptor {
        //     label: Some("game view a"),
        //     format: Some(TextureFormat::R32Float),
        //     base_mip_level: 0,
        //     mip_level_count: Some(1),
        //     ..Default::default()
        // });

        // let input_view_buff = device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("which is input"),
        //     contents: bytemuck::cast_slice(&[0]),
        //     usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        // });

        // let bind_group = device.create_bind_group(&BindGroupDescriptor {
        //     label: Some("storage bind group"),
        //     layout: &bind_group_layout,
        //     entries: &[
        //         BindGroupEntry {
        //             binding: 0,
        //             resource: BindingResource::TextureView(&game_view_a),
        //         },
        //         BindGroupEntry {
        //             binding: 1,
        //             resource: BindingResource::TextureView(&game_view_b),
        //         },
        //         BindGroupEntry {
        //             binding: 2,
        //             resource: input_view_buff.as_entire_binding(),
        //         },
        //     ],
        // });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[
                // &bind_group_layout
            ],
            push_constant_ranges: &[],
        });

        // let init_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        //     label: Some("lenia init"),
        //     layout: Some(&layout),
        //     module: &shader,
        //     entry_point: "init",
        // });

        // let update_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        //     label: Some("lenia compute"),
        //     layout: Some(&layout),
        //     module: &shader,
        //     entry_point: "update",
        // });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            // update_pipeline,
            // init_pipeline,
            render_pipeline,
            // game_buff_a,
            // game_buff_b,
            // bind_group,
        }
    }

    pub fn setup_board(&self, encoder: &mut wgpu::CommandEncoder) {
        {
            // let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            // pass.set_bind_group(0, &self.bind_group, &[]);
            // pass.set_pipeline(&self.init_pipeline);
            // pass.dispatch_workgroups(GAME_SIZE / WORKGROUP_SIZE, GAME_SIZE / WORKGROUP_SIZE, 1);
        }
    }

    pub fn calc_step(&self, encoder: &mut wgpu::CommandEncoder) {
        // let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        //     label: None,
        //     timestamp_writes: None,
        // });
        // pass.set_pipeline(&self.update_pipeline);
        todo!()
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &TextureView) {
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            // pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_pipeline(&self.render_pipeline);
            pass.draw(0..3, 0..1);
        }
    }
}
