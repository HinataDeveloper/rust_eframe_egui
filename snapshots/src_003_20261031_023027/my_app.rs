use eframe::egui;

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is heading ...");
            ui.label("This is label ...");
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye ...");
    }
}

/// # Errors
///
/// ``display_my_app()`` function returns an ``eframe::Error``
/// if encounter with any problem.
pub fn display_my_app() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_title("eframe (egui)"),
        ..Default::default()
    };

    eframe::run_native(
        "My First App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )?;
    Ok(())
}
