use crate::{
    renderer::{RenderData, Renderer},
    vertex::Vertex,
};
use eframe::egui;
use std::sync::Arc;

pub struct App {
    render_data: RenderData,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            render_data: RenderData {
                format: cc.wgpu_render_state.as_ref().unwrap().target_format,
                objects: vec![
                    Vertex {
                        pos: [0.0, 0.5, 0.0],
                        col: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        pos: [-0.5, -0.5, 0.0],
                        col: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        pos: [0.5, -0.5, 0.0],
                        col: [0.0, 0.0, 1.0],
                    },
                ],
            },
        }
    }

    fn custom_paint(&mut self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::splat(500.0), egui::Sense::click());
        let render_data = Arc::new(self.render_data.clone());

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(
                eframe::egui_wgpu::CallbackFn::default()
                    .prepare(move |device, _queue, map| {
                        Renderer::prepare(device, map, render_data.clone());
                    })
                    .paint(|_info, render_pass, map| {
                        Renderer::paint(render_pass, map);
                    }),
            ),
        };
        ui.painter().add(callback);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.label("Canvas test");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_paint(ui);
            })
        });

        ctx.request_repaint();
    }
}
