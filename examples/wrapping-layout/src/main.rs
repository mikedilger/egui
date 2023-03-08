use eframe::{
    egui::{self, TextFormat},
    epaint::text::LayoutJob,
};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.horizontal_wrapped(|ui| {
                ui.hyperlink_to("@npub1vdaeclr2mnntmyw...", "whocares");
                let text = " lnbc10u1p3lz4dppp5dsj2mh5kgqfqqxwhkrkw60stn8aph4gm2h2053xvwvvlvjm3q9eqdpqxycrqvpqd3hhgar9wfujqarfvd4k2arncqzpgxqzz6sp5vfenc5l4uafsky0w069zs329edf608ggpjjveguwxfl3xlswg5vq9qyyssqj46d5x3gsnljffm79eqwszk4mk47lkxywdp8mxum7un3qm0ztwj9jf46cm4lw2un9hk4gttgtjdrk29h27xu4e3ume20sqsna8q7xwspqqkwq7";
                let job = LayoutJob::single_section(text.to_owned(), TextFormat::default());
                ui.label(job);
           });
       });
    }
}
