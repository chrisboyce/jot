#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod easy_mark;
pub use app::JotApp;

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct EasyMarkApp {
    editor: easy_mark::EasyMarkEditor,
}

impl eframe::App for EasyMarkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.editor.panels(ctx);
    }
}
