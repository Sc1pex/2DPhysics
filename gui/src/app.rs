use eframe::{egui, egui_wgpu::RenderState};

pub struct App {
    render_state: RenderState,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            render_state: cc.wgpu_render_state.as_ref().unwrap().clone()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("opti").show(ctx, |ui| {
            ui.label("Samples per pixel: ");
            ui.label("Max ray bounces: ");
        });

        ctx.request_repaint();
    }
}
