use std::collections::HashMap;

use houselab_core::{
    Comment, Severity,
    inspection::Inspection,
    template::{Id, Section},
    person::People,
};

use crate::interp;

pub fn details(inspection: &mut Inspection, people: &People, ui: &mut egui::Ui) {
    ui.heading("Inspection Details");

    ui.label("Inspection Name:");
    ui.text_edit_singleline(&mut inspection.name);

    ui.label("Address:");
    ui.text_edit_singleline(&mut inspection.address);

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Date:");
        ui.add(egui_extras::DatePickerButton::new(&mut inspection.date));
    });

    horiz("Time: ", inspection.time.to_string(), ui);

    ui.add_space(10.0);
    horiz("Template:", &inspection.template.name, ui);

    ui.add_space(10.0);
    horiz("# Images:", inspection.images.len().to_string(), ui);

    ui.separator();
    ui.heading("Client");

    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut inspection.client.info.name);
    });

    if let Some(phone) = &mut inspection.client.info.phone {
        ui.horizontal(|ui| {
            ui.label("Phone:");
            ui.text_edit_singleline(phone);
        });
    }

    if let Some(email) = &mut inspection.client.info.email {
        ui.horizontal(|ui| {
            ui.label("Email:");
            ui.text_edit_singleline(email);
        });
    }

    ui.horizontal(|ui| {
        ui.label("Realtor:");
        let mut delete = false;
        if let Some(realtor) = &mut inspection.client.realtor {
            if ui.button("Remove").clicked() {
                delete = true;
            }

            egui::ComboBox::from_id_salt("buyer-realtor-select")
                .selected_text(&realtor.info.name)
                .show_ui(ui, |ui| {
                    for option in &people.realtors {
                        ui.selectable_value(realtor, option.clone(), &option.info.name);
                    }
                });
        } else {
            if ui.button("Set").clicked() && let Some(realtor) = people.realtors.first() {
                inspection.client.realtor = Some(realtor.clone());
            }
        }
        if delete {
            inspection.client.realtor = None;
        }
    });

    ui.separator();
    ui.heading("Seller's Agent");
    let mut delete = false;
    if let Some(realtor) = &mut inspection.seller {
        ui.horizontal(|ui| {
            if ui.button("Remove").clicked() {
                delete = true;
            }

            egui::ComboBox::from_id_salt("seller-realtor-select")
                .selected_text(&realtor.info.name)
                .show_ui(ui, |ui| {
                    for option in &people.realtors {
                        ui.selectable_value(realtor, option.clone(), &option.info.name);
                    }
                });
        });
    } else {
        if ui.button("Set").clicked() && let Some(realtor) = people.realtors.first() {
            inspection.seller = Some(realtor.clone());
        }
    }
    if delete {
        inspection.seller = None;
    }

    ui.separator();
    ui.horizontal(|ui| {
        ui.heading("Inspectors");

        if ui.button("Add").clicked() {
            let Some(inspector) = people.inspectors.first() else {
                return;
            };
            inspection.inspectors.push(inspector.clone());
        }
    });

    let mut delete = None;
    for (i, inspector) in inspection.inspectors.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            if ui.button("X").clicked() {
                delete = Some(i);
            }

            egui::ComboBox::from_id_salt((i, &*inspector))
                .selected_text(&inspector.info.name)
                .show_ui(ui, |ui| {
                    for option in &people.inspectors {
                        ui.selectable_value(inspector, option.clone(), &option.info.name);
                    }
                });
        });
    }
    if let Some(i) = delete {
        inspection.inspectors.remove(i);
    }
}

fn horiz(label: impl Into<egui::WidgetText>, body: impl Into<egui::WidgetText>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(body);
    });
}

#[derive(Debug, PartialEq)]
pub enum Edit {
    MoveUp(Id),
    MoveDown(Id),
    Delete(Id),
    Sibling(Section),
    Child(Id, Section),
}

pub fn main_section(
    inspection: &mut Inspection,
    current_section: &mut Option<Id>,
    ui: &mut egui::Ui,
) {
    let Some(id) = *current_section else {
        panic!("`section::main` needs to be called with a `current_section` value of Some")
    };

    let mut id_gen = inspection.template.id_gen;
    let mut new_id = || {
        let id = Id::root(id_gen);
        id_gen += 1;
        log::info!("generated new root id {id}");
        Some(id)
    };
    let mut new_id: &mut dyn FnMut() -> Option<Id> = &mut new_id;

    let mut new_id_child;
    let mut parent_id = None;
    if let Some(parent) = id.parent()
        && let Some(parent) = inspection.template.get(parent)
    {
        parent_id = Some(parent.id);
        new_id_child = || {
            let id = parent_id.as_mut().unwrap().next();
            log::info!("generated new child id {id:?}");
            id
        };
        new_id = &mut new_id_child;
    }

    let Some(section) = inspection.template.get_mut(id) else {
        log::error!("invalid section id {id}");
        *current_section = None;
        return;
    };

    'edit: {
        match self::section(section, new_id, ui) {
            None => {}
            Some(Edit::MoveUp(id)) => {
                if let Some(parent) = id.parent() {
                    let Some(section) = inspection.template.get_mut(parent) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    let Some(i) = section.children.iter().position(|s| s.id == id) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    if i == 0 {
                        break 'edit;
                    }

                    log::info!("moving section {id} ({}) up", section.children[i].name);
                    section.children.swap(i, i - 1);
                } else {
                    let Some(i) = inspection.template.sections.iter().position(|s| s.id == id)
                    else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    if i == 0 {
                        break 'edit;
                    }

                    log::info!(
                        "moving section {id} ({}) up",
                        inspection.template.sections[i].name
                    );
                    inspection.template.sections.swap(i, i - 1);
                }
            }
            Some(Edit::MoveDown(id)) => {
                if let Some(parent) = id.parent() {
                    let Some(section) = inspection.template.get_mut(parent) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    let Some(i) = section.children.iter().position(|s| s.id == id) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    if i == section.children.len() - 1 {
                        break 'edit;
                    }

                    log::info!("moving section {id} ({}) down", section.children[i].name);
                    section.children.swap(i, i + 1);
                } else {
                    let Some(i) = inspection.template.sections.iter().position(|s| s.id == id)
                    else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    if i == inspection.template.sections.len() - 1 {
                        break 'edit;
                    }

                    log::info!(
                        "moving section {id} ({}) down",
                        inspection.template.sections[i].name
                    );
                    inspection.template.sections.swap(i, i + 1);
                }
            }
            Some(Edit::Delete(id)) => {
                if let Some(parent) = id.parent() {
                    let Some(section) = inspection.template.get_mut(parent) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    let Some(i) = section.children.iter().position(|s| s.id == id) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    log::info!("deleting section {id} ({})", section.children[i].name);
                    section.children.remove(i);
                    *current_section = id.parent();
                } else {
                    let Some(i) = inspection.template.sections.iter().position(|s| s.id == id)
                    else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    log::info!(
                        "deleting section {id} ({})",
                        inspection.template.sections[i].name
                    );
                    inspection.template.sections.remove(i);
                    *current_section = id.parent();
                }
            }
            Some(Edit::Sibling(section)) => {
                if let Some(parent) = section.id.parent() {
                    let Some(parent) = inspection.template.get_mut(parent) else {
                        log::error!("invalid section id {id}");
                        break 'edit;
                    };

                    log::info!(
                        "adding section {} ({}) to section {} ({})",
                        section.id,
                        section.name,
                        parent.id,
                        parent.name
                    );
                    *current_section = Some(section.id);
                    parent.children.push(section);
                } else {
                    log::info!("adding section {} ({}) to root", section.id, section.name,);
                    inspection.template.sections.push(section);
                }
            }
            Some(Edit::Child(parent, section)) => {
                let Some(parent) = inspection.template.get_mut(parent) else {
                    log::error!("invalid section id {id}");
                    break 'edit;
                };

                log::info!(
                    "adding section {} ({}) to section {} ({})",
                    section.id,
                    section.name,
                    parent.id,
                    parent.name
                );
                *current_section = Some(section.id);
                parent.children.push(section);
            }
        }
    }

    inspection.template.id_gen = id_gen;

    if let Some(id) = parent_id
        && let Some(section) = inspection.template.get_mut(id)
    {
        section.id = id;
    }
}

pub fn section(
    section: &mut Section,
    new_id: &mut dyn FnMut() -> Option<Id>,
    ui: &mut egui::Ui,
) -> Option<Edit> {
    ui.heading(&section.name);

    if let edit @ Some(_) = ui
        .horizontal(|ui| {
            ui.checkbox(&mut section.inline, "Inline");

            if ui.button("Move Up").clicked() {
                return Some(Edit::MoveUp(section.id));
            }

            if ui.button("Move Down").clicked() {
                return Some(Edit::MoveDown(section.id));
            }

            if ui.button("New Child").clicked() {
                if let Some(id) = section.id.next() {
                    let child = Section {
                        id,
                        name: "New Section".into(),
                        description: None,
                        inline: section.inline,
                        comments: Vec::new(),
                        observations: Vec::new(),
                        children: Vec::new(),
                    };
                    return Some(Edit::Child(section.id, child));
                } else {
                    log::error!("attempted to generate overly nested IDs");
                }
            }

            if ui.button("Duplicate").clicked() {
                let mut sibling = section.clone();
                if let Some(id) = new_id() {
                    sibling.rebase(id);
                    return Some(Edit::Sibling(sibling));
                } else {
                    log::error!("attempted to generate overly nested IDs");
                }
            }

            if ui.button("Delete").clicked() {
                // TODO: confirmation? undo?
                return Some(Edit::Delete(section.id));
            }

            None
        })
        .inner
    {
        return edit;
    }

    ui.label("Section Name:");
    ui.text_edit_singleline(&mut section.name);

    let mut delete = false;
    if let Some(description) = &mut section.description {
        ui.horizontal(|ui| {
            ui.label("Description:");
            if ui.button("Delete").clicked() {
                delete = true;
            }
        });

        ui.text_edit_multiline(description);
    } else {
        if ui.button("Add description").clicked() {
            section.description = Some(String::new());
        }
    }
    if delete {
        section.description = None;
    }

    ui.add_space(20.0);

    ui.horizontal(|ui| {
        ui.label("Comments:");

        if ui.button("Add").clicked() {
            section.comments.push(Comment {
                base: "New comment".into(),
                applied: true,
                lists: HashMap::new(),
                entries: HashMap::new(),
                severity: Severity::General,
                summary: false,
            });
        }
    });

    let mut delete = None;
    for (i, comment) in section.comments.iter_mut().enumerate() {
        if interp::comment((section.id, i), comment, ui) {
            delete = Some(i);
        }
    }
    if let Some(i) = delete {
        section.comments.remove(i);
    }

    ui.indent(section.id, |ui| {
        for child in section.children.iter_mut().filter(|s| s.inline) {
            ui.separator();
            if let edit @ Some(_) = self::section(
                child,
                &mut || {
                    let id = section.id.next();
                    log::info!("generated new child id {id:?}");
                    id
                },
                ui,
            ) {
                return edit;
            }
        }

        None
    })
    .inner
}
