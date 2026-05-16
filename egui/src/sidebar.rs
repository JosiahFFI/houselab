use houselab_core::template::{Id, Section};

pub fn item(current_section: &mut Option<Id>, section: &Section, ui: &mut egui::Ui) {
    let button = if *current_section == Some(section.id) {
        ui.button(egui::RichText::new(&section.name).strong().underline())
    } else {
        ui.button(&section.name)
    };

    if button.clicked() {
        log::info!("opening section {} ({})", section.id, section.name);
        *current_section = Some(section.id);
    }

    for child in &section.children {
        ui.indent(child.id, |ui| item(current_section, child, ui));
    }
}
