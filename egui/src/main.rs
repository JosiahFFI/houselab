fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Houselab",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            houselab_egui::style::desktop(&cc.egui_ctx);

            Ok(Box::new(houselab_egui::Houselab {
                people: houselab_core::test::people(),
                ..Default::default()
            }))
        }),
    )
}
