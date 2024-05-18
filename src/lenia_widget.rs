use iced::{
    mouse,
    widget::shader::{
        wgpu::{
            self, include_wgsl,
            util::{BufferInitDescriptor, DeviceExt},
            BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingResource,
            BufferUsages, ComputePipeline, RenderPipeline, ShaderStages, Texture,
            TextureDescriptor, TextureFormat, TextureUsages, TextureViewDescriptor,
        },
        Primitive, Program,
    },
};

#[derive(Debug)]
pub struct LeniaPrimitive {}

impl Primitive for LeniaPrimitive {
    fn prepare(
        &self,
        format: iced::widget::shader::wgpu::TextureFormat,
        device: &iced::widget::shader::wgpu::Device,
        queue: &iced::widget::shader::wgpu::Queue,
        _bounds: iced::Rectangle,
        target_size: iced::Size<u32>,
        _scale_factor: f32,
        storage: &mut iced::widget::shader::Storage,
    ) {
        if !storage.has::<LeniaPipeline>() {
            storage.store(LeniaPipeline::new(device, queue, format));
        }
        todo!()
        // put the pipepline in here for setting up the random board etc
    }

    fn render(
        &self,
        storage: &iced::widget::shader::Storage,
        target: &iced::widget::shader::wgpu::TextureView,
        _target_size: iced::Size<u32>,
        _viewport: iced::Rectangle<u32>,
        encoder: &mut iced::widget::shader::wgpu::CommandEncoder,
    ) {
        let lenia_compute = storage.get::<LeniaPipeline>().unwrap();
        lenia_compute.calc_step(encoder);

        todo!()
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
    init_pipeline: ComputePipeline,
    update_pipeline: ComputePipeline,
    render_pipeline: RenderPipeline,
    game_buff_a: Texture,
    game_buff_b: Texture,
}

impl LeniaPipeline {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/lenia.wgsl"));
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let game_buff_a = device.create_texture(&TextureDescriptor {
            label: Some("game buff a"),
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 0,
            },
            mip_level_count: 0,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::R32Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let game_buff_b = device.create_texture(&TextureDescriptor {
            label: Some("game buff b"),
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 0,
            },
            mip_level_count: 0,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::R32Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let game_view_a = game_buff_a.create_view(&TextureViewDescriptor {
            label: Some("game view a"),
            format: Some(TextureFormat::R32Float),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });
        let game_view_b = game_buff_b.create_view(&TextureViewDescriptor {
            label: Some("game view a"),
            format: Some(TextureFormat::R32Float),
            base_mip_level: 0,
            mip_level_count: Some(1),
            ..Default::default()
        });

        let input_view_buff = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("which is input"),
            contents: bytemuck::cast_slice(&[0]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("storage bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&game_view_a),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&game_view_b),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: input_view_buff.as_entire_binding(),
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let init_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("lenia init"),
            layout: Some(&layout),
            module: &shader,
            entry_point: "init",
        });

        let update_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("lenia compute"),
            layout: Some(&layout),
            module: &shader,
            entry_point: "update",
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("lenia render"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "render_vs",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "render_fs",
                targets: &[],
            }),
            multiview: None,
        });

        Self {
            update_pipeline,
            init_pipeline,
            render_pipeline,
            game_buff_a,
            game_buff_b,
        }
    }

    pub fn setup_board() {
        todo!()
    }

    pub fn calc_step(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.update_pipeline);
        todo!()
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor::default());
            pass.set_pipeline(&self.render_pipeline);
            let view = self
                .game_buff_a
                .create_view(&TextureViewDescriptor::default());
        }
        todo!()
    }
}

struct LeniaRender {}

impl LeniaRender {}
