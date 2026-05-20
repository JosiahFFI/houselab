use houselab_core::person::{Inspector, Person, Realtor};

pub fn main(app: &mut crate::Houselab, ui: &mut egui::Ui) {
    ui.heading("Inspectors");

    ui.add_space(5.0);
    if ui.button("Add").clicked() {
        app.people.inspectors.push(Inspector {
            info: Person {
                name: "New Inspector".into(),
                phone: None,
                email: None,
            },
            licenses: vec![],
        });

        return;
    }

    ui.add_space(20.0);
    let mut delete = None;
    for (i, inspector) in app.people.inspectors.iter_mut().enumerate() {
        ui.indent(i, |ui| {
            if ui.button("Delete").clicked() {
                delete = Some(i);
                return;
            }
            ui.add_space(10.0);

            ui.label("Name:");
            ui.text_edit_singleline(&mut inspector.info.name);

            ui.add_space(10.0);
            ui.label("Phone:");
            let mut delete = false;
            if let Some(phone) = &mut inspector.info.phone {
                ui.text_edit_singleline(phone);

                if ui.button("Remove").clicked() {
                    delete = true;
                }
            } else if ui.button("Set").clicked() {
                inspector.info.phone = Some(String::new());
            }
            if delete {
                inspector.info.phone = None;
            }

            ui.add_space(10.0);
            ui.label("Email:");
            let mut delete = false;
            if let Some(email) = &mut inspector.info.email {
                ui.text_edit_singleline(email);

                if ui.button("Remove").clicked() {
                    delete = true;
                }
            } else if ui.button("Set").clicked() {
                inspector.info.email = Some(String::new());
            }
            if delete {
                inspector.info.email = None;
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Licenses:");

                if ui.button("Add").clicked() {
                    inspector.licenses.push((String::new(), String::new()));
                }
            });

            let mut delete = None;
            for (i, (name, license)) in inspector.licenses.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button("X").clicked() {
                        delete = Some(i);
                    }

                    ui.text_edit_singleline(name);
                    ui.text_edit_singleline(license);
                });
            }
            if let Some(name) = delete {
                inspector.licenses.remove(name);
            }
        });

        ui.add_space(20.0);
    }
    if let Some(i) = delete {
        app.people.inspectors.remove(i);
    }

    ui.separator();
    ui.add_space(20.0);
    ui.heading("Realtors");

    ui.add_space(5.0);
    if ui.button("Add").clicked() {
        app.people.realtors.push(Realtor {
            info: Person {
                name: "New Realtor".into(),
                phone: None,
                email: None,
            },
            firm: "Realty Firm".into(),
        });

        return;
    }

    ui.add_space(20.0);
    let mut delete = None;
    for (i, realtor) in app.people.realtors.iter_mut().enumerate() {
        ui.indent(i, |ui| {
            if ui.button("Delete").clicked() {
                delete = Some(i);
                return;
            }
            ui.add_space(10.0);

            ui.label("Name:");
            ui.text_edit_singleline(&mut realtor.info.name);

            ui.add_space(10.0);
            ui.label("Phone:");
            let mut delete = false;
            if let Some(phone) = &mut realtor.info.phone {
                ui.text_edit_singleline(phone);

                if ui.button("Remove").clicked() {
                    delete = true;
                }
            } else if ui.button("Set").clicked() {
                realtor.info.phone = Some(String::new());
            }
            if delete {
                realtor.info.phone = None;
            }

            ui.add_space(10.0);
            ui.label("Email:");
            let mut delete = false;
            if let Some(email) = &mut realtor.info.email {
                ui.text_edit_singleline(email);

                if ui.button("Remove").clicked() {
                    delete = true;
                }
            } else if ui.button("Set").clicked() {
                realtor.info.email = Some(String::new());
            }
            if delete {
                realtor.info.email = None;
            }

            ui.add_space(10.0);
            ui.label("Firm:");
            ui.text_edit_singleline(&mut realtor.firm);
        });

        ui.add_space(20.0);
    }
    if let Some(i) = delete {
        app.people.realtors.remove(i);
    }
}
