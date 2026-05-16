use dioxus::prelude::*;

use crate::INSPECTION;

#[component]
pub fn InspectionInfo() -> Element {
    let inspection = crate::INSPECTION.write();

    let date = inspection.date.to_string();
    let time = format!("{:02}:{:02}", inspection.time.hour(), inspection.time.minute());

    rsx! {
        div { class: "w-2/3 p-8 overflow-y-auto", id: "workspace",
            h1 { "{inspection.name} Info" }
            hr {}
            br {}

            label { "Inspection Name:" }
            input {
                type: "text",
                class: "bg-white rounded-sm p-1 m-1",
                value: "{inspection.name}",
                oninput: move |event| INSPECTION.write().name = event.value(),
            }

            br {}
            label { "Inspection Date:" }
            input {
                type: "date",
                class: "bg-white fg-black rounded-sm p-1 m-1",
                value: "{date}",
                oninput: move |event| -> () {
                    let mut inspection = INSPECTION.write();
                    let Ok(date) = event.value().parse() else {
                        return
                    };
                    inspection.date = date;
                },
            }
            input {
                type: "time",
                class: "bg-white rounded-sm p-1 m-1",
                value: "{time}",
                oninput: move |event| {
                    let mut inspection = INSPECTION.write();
                    let Ok(date) = format!("{}:00-00", event.value()).parse() else {
                        return
                    };
                    inspection.date = date;
                },
            }

            // TODO: add a proper "Inspectors" tab to change preset inspectors,
            // then add the ability to edit assigned inspectors here
            br {}
            label { "Inspectors:" }
            ol {
                for (i, inspector) in inspection.inspectors.iter().enumerate() {
                    "{i+1}. {inspector.info.name}"
                    {if let Some(phone) = &inspector.info.phone {
                        format!(" [{phone}]")
                    } else if let Some(email) = &inspector.info.email {
                        format!(" [{email}]")
                    } else { String::new() }}
                }
            }

            // TODO: add a proper "Clients" tab to change preset clients,
            // then add the ability to edit assigned client here
            br {}
            label { "Client:" }
            " {inspection.client.info.name}"
            {if let Some(phone) = &inspection.client.info.phone {
                format!(" [{phone}]")
            } else if let Some(email) = &inspection.client.info.email {
                format!(" [{email}]")
            } else { String::new() }}
        }
    }
}
