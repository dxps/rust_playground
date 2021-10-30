mod headlines;

use eframe::{
    egui::{
        CentralPanel, CtxRef, Hyperlink, Label, ScrollArea, Separator, TopBottomPanel, Ui, Vec2,
    },
    epi::App,
    run_native, NativeOptions,
};
use headlines::{Headlines, PADDING};

fn main() {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600.0, 900.0));
    run_native(Box::new(app), win_options)
}

impl App for Headlines {
    // A one-time lifecycle method called during initial init.
    fn setup(
        &mut self,
        ctx: &CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            // render_header(ui);
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| self.render_news_cards(ui));
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn _render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| ui.heading("Headlines"));
    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.));
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org").monospace());
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("Made with egui")
                    .text_style(eframe::egui::TextStyle::Monospace),
            );
            ui.add_space(10.);
        });
    });
}
