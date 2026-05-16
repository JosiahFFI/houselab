use dioxus::prelude::*;

use houselab_core::template::{Id, Section};

#[component]
pub fn Sidebar(current: Signal<Id>) -> Element {
    // let mut force_collapsed = use_signal(|| false);
    let inspection = crate::INSPECTION.read();

    rsx! {
        div { class: "w-1/3 p-8 border-r bg-white flex flex-col shadow-lg", id: "sidebar",
            // div { class: "m-1 p-1 pl-3 bg-gray-100", onclick: move |_| { force_collapsed.set(true); },
            //     "Collapse all"
            // }
            ul {
                li { class: "sidebar-listing w-fit m-1 p-1 pl-3 pr-3 rounded-sm bg-gray-100",
                    onclick: move |_| -> () { todo!() },
                    key: "inspection-info",

                    "{inspection.name}"
                }
                p { class: "p-1" }

                for section in inspection.template.sections.iter() {
                    li { key: "{section.name}",
                        Listing {
                            section: section.clone(),
                            id: section.id,
                            current,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Listing(section: Section, id: Id, current: Signal<Id>) -> Element {
    let mut collapsed = use_signal(|| false);
    let marker = if collapsed() {
        "+"
    } else {
        "-"
    };

    rsx! {
        div { class: "",
            span { class: "sidebar-listing w-fit m-1 mr-3 p-1 pl-3 pr-3 rounded-sm bg-gray-100",
                onclick: move |_| { collapsed.set(!collapsed()); },

                b{ if !section.children.iter().any(|c| !c.inline) { ">" } else { {marker} } }
            }   

            span { class: "sidebar-listing w-fit m-1 p-1 pl-3 pr-3 rounded-sm bg-gray-100",
                onclick: move |_| { current.set(id); },

                " {section.name}"
            }

            p { class: "p-2" }

            if !collapsed() {
                ul { style: "padding-left: 1.5ch",
                    for child in section.children.iter().filter(|c| !c.inline) {
                        li { key: "{child.id}",
                            Listing {
                                section: child.clone(),
                                id: child.id,
                                current,
                            }
                        }
                    }
                }
            }
        }
    }
}
