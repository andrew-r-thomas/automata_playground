use iced::{
    mouse,
    widget::shader::{
        wgpu::{
            self, include_wgsl, BindGroupLayoutEntry, ComputePipeline, RenderPipeline,
            ShaderStages, TextureFormat,
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
}

impl LeniaPipeline {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/lenia.wgsl"));
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
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
            // TODO
        });

        Self {
            update_pipeline,
            init_pipeline,
            render_pipeline,
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
}

struct LeniaRender {}

impl LeniaRender {}
