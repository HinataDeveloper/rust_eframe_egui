// lib.rs
// Date: Thu Oct 29 2026
// Educational and Practice Rust Programming Language Code

// Dependency: Depend on to the eframe crate

// rustc 1.101.0-nightly (92ffebece 2026-10-28)
// binary: rustc
// commit-hash: 92ffebeceaa0a071066f233873c2438e73d70fe7
// commit-date: 2026-10-28
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.4

// cargo 1.101.0-nightly (cad07cf8e 2026-10-23)
// release: 1.101.0-nightly
// commit-hash: cad07cf8e597397fff5d0f6cb79c9f870c0b1dd2
// commit-date: 2026-10-23
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.13-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use eframe::egui::{self};

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This the Head ...");
            ui.label("This the label ...")
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye Hinata ...");
    }
}

/// # Errors
///
/// ``display_my_app`` function returns ``eframe::Error``
/// if encunter with any problem amoung of
/// its operation.
pub fn display_my_app() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("eframe (egui)"),
        ..Default::default()
    };

    eframe::run_native(
        "My First Application",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )?;
    Ok(())
}
