use crate::vertex::Vertex;
use eframe::wgpu::{self, util::DeviceExt};
use std::sync::Arc;
use type_map::concurrent::TypeMap;

#[derive(Clone)]
pub struct RenderData {
    pub format: wgpu::TextureFormat,
    pub objects: Vec<Vertex>,
}

pub struct Renderer {}

impl Renderer {
    pub fn prepare(device: &wgpu::Device, map: &mut TypeMap, data: Arc<RenderData>) {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: data.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&data.objects),
            usage: wgpu::BufferUsages::VERTEX,
        });

        map.insert(render_pipeline);
        map.insert(vertex_buffer);
    }

    pub fn paint<'a>(render_pass: &mut wgpu::RenderPass<'a>, map: &'a TypeMap) {
        render_pass.set_pipeline(map.get().unwrap());
        render_pass.set_vertex_buffer(0, map.get::<wgpu::Buffer>().unwrap().slice(..));
        render_pass.draw(0..3, 0..1);
    }
}
