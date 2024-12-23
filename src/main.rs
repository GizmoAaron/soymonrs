use ddc_hi::{Ddc,Display};
use mccs_db::{ValueType};
use egui::{CentralPanel, ComboBox, Context, Ui};

pub struct Tuple {
    display: Display,
    sources: Vec<Source>
}
pub struct Source {
    hex: u8,
    name: String
}
// index in array is the id of the display
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "Emoji Dropdown UI",
        options,
        Box::new(|cc| {
            Ok(Box::new(App::new(cc)))
        }),
    );
    Ok(())
}
struct App{
    display_data: Vec<Tuple>
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            display_data: grab_displays()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        configure_style(ctx,48.0);
        CentralPanel::default().show(ctx, |ui| {
            render_ui(ui, &mut self.display_data);
        });
    }
}

fn configure_style(ctx: &egui::Context, size:f32) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(size));
    style.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(size)); // Used in Selectable
    ctx.set_style(style);
}

pub fn render_ui(ui: &mut Ui, ui_elements: &mut Vec<Tuple>) {
    for component in ui_elements.iter_mut() {
        ui.horizontal(|ui| {
            // Add a dropdown panel (ComboBox) on the right
            ComboBox::from_label(component.display.info.model_name.as_ref().unwrap())
                .selected_text("Select an Input")
                .show_ui(ui, |ui| {
                    for input in &component.sources {
                        if ui.selectable_value(
                        &mut input.name.as_str(), 
                        input.name.as_str(), 
                        input.name.as_str())
                        .clicked(){
                            component.display.handle.set_vcp_feature(0x60, input.hex as u16);
                        };
                    }
                }
            );
        });
    }
}

fn grab_displays() -> Vec<Tuple> {
    println!("Grabbing Displays.");
    let mut display_values: Vec<Tuple> = Vec::new();
    for mut display in Display::enumerate() {
        display.update_capabilities().unwrap();
        if display.info.backend.to_string().contains("win") {
            if let Some(feature) = display.info.mccs_database.get(0x60) {
                let mut tuples: Vec<Source> = Vec::new();
                match feature.ty {
                    ValueType::NonContinuous { ref values, .. } =>
                        for (value, name) in values {
                            let source = Source {hex: *value, name: name.as_ref().unwrap().clone()};
                            tuples.push(source);
                        },
                    _ => (),
                }
                let dis = Tuple{display: display,sources: tuples};
                display_values.push(dis);
            }
        }
    }
    // let mut dp1 = &mut display_values[0];
    // dp1.display.handle.set_vcp_feature(0x60, dp1.sources[2].hex as u16);
    return display_values;
}