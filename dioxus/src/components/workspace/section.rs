use dioxus::prelude::*;

use houselab_core::template::Id;
use crate::INSPECTION;

#[component]
pub fn Section(id: Id) -> Element {
    let inspection = crate::INSPECTION.read();

    let Some(section) = inspection.template.get(id) else {
        return rsx! {
            b{ "Error: invalid path" }
        };
    };

    rsx! {
        div { class: "w-2/3 pl-1",
            h1 { "{section.name}" }

            label { "Section Name:" }
            input {
                type: "text",
                class: "bg-white rounded-sm p-1 m-1",
                value: "{section.name}",
                oninput: move |event| {
                    let mut inspection = INSPECTION.write();
                    let Some(section) = inspection.template.get_mut(id) else {
                        return;
                    };
                    section.name = event.value();
                }
            }

            for child in section.children.iter().filter(|c| c.inline) {
                Section { id: child.id }
            }
        }
    }
}
