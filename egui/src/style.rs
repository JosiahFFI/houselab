#[cfg(target_os = "android")]
pub fn android(ctx: &egui::Context) {
    use egui::FontFamily::Proportional;
    use egui::FontId;
    use egui::TextStyle::*;
    use std::collections::BTreeMap;

    let text_styles: BTreeMap<_, _> = [
        (Heading, FontId::new(30.0, Proportional)),
        (Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (Name("Context".into()), FontId::new(23.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();

    ctx.all_styles_mut(move |style| {
        style.text_styles = text_styles.clone();

        style.spacing.scroll = egui::style::ScrollStyle::thin();
        style.spacing.item_spacing = egui::Vec2 { x: 12.0, y: 8.0 };
        style.spacing.button_padding = egui::Vec2 { x: 8.0, y: 4.0 };
        style.spacing.icon_width = 24.0;
    });
}

#[cfg(not(target_os = "android"))]
pub fn desktop(ctx: &egui::Context) {
    ctx.all_styles_mut(move |style| {
        style.spacing.scroll = egui::style::ScrollStyle::thin();
    });
}
