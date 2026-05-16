pub fn main(app: &mut crate::Houselab, ui: &mut egui::Ui) {
    if let Some(inspection) = &app.inspection {
        ui.heading("Current Inspection");
        ui.label(&inspection.name);

        if ui.button("Close Inspection").clicked() {
            app.inspection = None;
        }
    } else {
        ui.heading("No Inspection Opened");

        if ui.button("Open Inspection").clicked() {
            app.inspection = Some(houselab_core::test::inspection());
        }
    }
}
