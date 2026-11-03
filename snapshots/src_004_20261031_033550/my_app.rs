use eframe::egui;
use std::fmt::{Debug, Display};

pub struct MyApp {
    name: String,
    counter: i32,
}

impl MyApp {
    #[must_use]
    pub fn new(name: impl Into<String>, counter: i32) -> Self {
        Self {
            name: name.into(),
            counter,
        }
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub const fn set_counter(&mut self, counter: i32) {
        self.counter = counter;
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn counter(&self) -> i32 {
        self.counter
    }
}

//------------------------------------------------------

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Hello Hinata".to_owned(),
            counter: 0,
        }
    }
}

//------------------------------------------------------

impl Display for MyApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[name:{}, counter:{}]", self.name, self.counter)
    }
}

//------------------------------------------------------

impl Debug for MyApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[name:{}, counter:{}]", self.name, self.counter)
    }
}

//------------------------------------------------------

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("State management on eframe.");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Your Name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add_space(8.0);

            if ui.button("Increment +").clicked() {
                self.counter += 1;
            }

            if ui.button("Decrement -").clicked() {
                self.counter -= 1;
            }

            if ui.button("Reset").clicked() {
                self.counter = 0;
            }

            ui.label(format!(
                "Hello {}! Your counter is on {}",
                self.name, self.counter
            ));

            ui.label(format!("counter: {}", self.counter));
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye ...");
    }
}

//------------------------------------------------------
/// # Errors
///
/// ``display_my_app()`` function returns ``eframe::Error``
/// if encounter with any problem in amoung ot its operation.
pub fn display_my_app(app: MyApp) -> Result<(), eframe::Error> {
    // Initialize app_name variable.
    let app_name = "Raphael Application";

    // Initialize native_options variable.
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_title("eframe (egui)"),
        ..Default::default()
    };

    eframe::run_native(app_name, native_options, Box::new(|_cc| Ok(Box::new(app))))?;
    Ok(())
}
