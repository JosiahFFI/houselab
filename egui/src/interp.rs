use houselab_core::Comment;

pub fn comment(
    salt: impl std::hash::Hash + Copy,
    comment: &mut Comment,
    ui: &mut egui::Ui,
) -> bool {
    ui.horizontal(|ui| {
        egui::ScrollArea::horizontal()
            .id_salt(salt)
            .show(ui, |ui| {
                if ui.button("X").clicked() {
                    return true;
                }

                let id = ui.make_persistent_id(salt);
                let mut is_editing = ui.data_mut(|d| *d.get_temp_mut_or_default::<bool>(id));
                ui.toggle_value(&mut is_editing, "[E]");
                ui.data_mut(|d| d.insert_temp(id, is_editing));

                if is_editing {
                    ui.text_edit_singleline(&mut comment.base);
                    return false;
                }

                ui.add_space(5.0);
                ui.toggle_value(&mut comment.summary, "[!]");

                if !comment.base.contains("{{") {
                    ui.checkbox(&mut comment.applied, &comment.base);
                    return false;
                } else if comment.base.matches("{{").count() != comment.base.matches("}}").count() {
                    log::error!("invalid comment: `{}`", comment.base);
                    ui.checkbox(&mut comment.applied, &comment.base);
                    return false;
                }

                ui.checkbox(&mut comment.applied, "");
                let mut base = comment.base.as_str();
                while let Some((start, rest)) = base.split_once("{{") {
                    ui.label(start);

                    let (name, rest) = rest.split_once("}}").unwrap();
                    if let Some(list) = comment.lists.get_mut(name) {
                        egui::ComboBox::from_id_salt(salt)
                            .selected_text(&list.items[list.selected])
                            .show_ui(ui, |ui| {
                                for (i, item) in list.items.iter().enumerate() {
                                    ui.selectable_value(&mut list.selected, i, item);
                                }
                            });
                    } else if let Some(entry) = comment.entries.get_mut(name) {
                        ui.text_edit_singleline(entry);
                    } else {
                        log::error!("invalid comment: couldn't find variable `{name}`");
                        ui.label(egui::RichText::new(format!("{{{{{name}}}}}")).strong());
                    }

                    base = rest;
                }
                ui.label(base);

                false
            })
            .inner
    })
    .inner
}
