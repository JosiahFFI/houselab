use dioxus::prelude::*;

use houselab_core::template::Id;

mod info;
mod section;

use info::InspectionInfo;
use section::Section;

#[component]
pub fn Workspace(current: Signal<Id>) -> Element {
    let is_section = true;

    rsx! {
        if is_section {
            div { class: "p-8 overflow-y-auto",
                Section { id: current() }
            }
        } else { InspectionInfo {} }
    }
}
