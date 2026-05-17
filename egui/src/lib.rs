#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

mod sidebar;

mod images;
mod inspection;
mod manage;
mod people;

mod interp;
pub mod style;

#[allow(unused)]
use houselab_core::{
    Color, Comment, List, Severity,
    inspection::Inspection,
    person::{People, Client, Inspector, Person, Realtor},
    template::{Id, Section, Template},
};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "Houselab",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            style::android(&cc.egui_ctx);

            Ok(Box::new(Houselab {
                people: houselab_core::test::people(),
                ..Default::default()
            }))
        }),
    )
    .unwrap()
}

#[derive(Debug, Default)]
pub struct Houselab {
    pub inspection: Option<Inspection>,
    pub current_section: Option<Id>,
    pub current_page: Page,

    pub people: People,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Manage,
    Inspection,
    People,
    Images,
}

impl eframe::App for Houselab {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "android")]
        egui::Panel::top("status_bar_space").show_inside(ui, |ui| ui.set_height(32.0));

        egui::Panel::top("header").show_inside(ui, |ui| self.header(ui));
        egui::Panel::left("sidebar").show_inside(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default().show_inside(ui, |ui| self.workspace(ui));
    }
}

impl Houselab {
    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(inspection) = &self.inspection {
                ui.heading(&inspection.name);
            } else {
                ui.heading("No Inspection Opened");
            }

            match self.current_page {
                Page::Manage => {
                    ui.heading("> Manage");
                }
                Page::Inspection => {
                    if let Some(inspection) = &self.inspection
                        && let Some(id) = self.current_section
                    {
                        if let Some(path) = inspection.template.path(id) {
                            for name in path {
                                ui.heading(">");
                                ui.heading(name);
                            }
                        } else {
                            log::error!("invalid section ID: {id}");
                        }
                    } else {
                        ui.heading("> Details");
                    }
                }
                Page::People => {
                    ui.heading("> People");
                }
                Page::Images => {
                    ui.heading("> Images");
                }
            }

            // This might be a little hacky, but it's the officially endorsed method.
            // Since it's now RTL, the items need to be in reverse order.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                let width = ui.ctx().content_rect().width();
                if width > 700.0 {
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Images").clicked() {
                        self.current_page = Page::Images;
                    }
                    if ui.button("People").clicked() {
                        self.current_page = Page::People;
                    }
                    if ui.button("Inspection").clicked() {
                        self.current_page = Page::Inspection;
                    }
                    if ui.button("Manage").clicked() {
                        self.current_page = Page::Manage;
                    }
                } else {
                    // RTL doesn't extend into menus, so this needs to be normally ordered
                    egui::Popup::menu(&ui.button("Menu")).show(|ui| {
                        if ui.button("Manage").clicked() {
                            self.current_page = Page::Manage;
                        }
                        if ui.button("Inspection").clicked() {
                            self.current_page = Page::Inspection;
                        }
                        if ui.button("People").clicked() {
                            self.current_page = Page::People;
                        }
                        if ui.button("Images").clicked() {
                            self.current_page = Page::Images;
                        }
                        if ui.button("Exit").clicked() {
                            std::process::exit(0);
                        }
                    });
                }
            });
        });
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let Some(inspection) = &mut self.inspection else {
            return;
        };
        if self.current_page != Page::Inspection {
            return;
        }

        egui::ScrollArea::vertical()
            .id_salt("sidebar")
            .show(ui, |ui| {
                let button = if self.current_section.is_none() {
                    ui.button(
                        egui::RichText::new("Inspection Details")
                            .strong()
                            .underline(),
                    )
                } else {
                    ui.button("Inspection Details")
                };

                if button.clicked() {
                    self.current_section = None;
                }

                for section in &inspection.template.sections {
                    sidebar::item(&mut self.current_section, section, ui);
                }
            });
    }

    fn workspace(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_salt("workspace")
            .show(ui, |ui| match self.current_page {
                Page::Manage => manage::main(self, ui),
                Page::Inspection => {
                    let Some(inspection) = &mut self.inspection else {
                        ui.heading("No Inspection Opened");
                        return;
                    };

                    if self.current_section.is_some() {
                        inspection::main_section(inspection, &mut self.current_section, ui);
                    } else {
                        inspection::details(inspection, &self.people, ui);
                    }
                }
                Page::People => people::main(self, ui),
                Page::Images => {}
            });
    }
}
