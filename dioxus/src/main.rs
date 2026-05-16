use dioxus::prelude::*;

use components::{Sidebar, Workspace};

use houselab_core::inspection::Inspection;

/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const INSPECTION: GlobalSignal<Inspection> = Signal::global(houselab_core::test::inspection);

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let current = use_signal(|| INSPECTION.read().template.sections[0].id);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "flex w-full bg-gray-150 border-b p-2",
            h2 {
                b { "Current Inspection: " } {INSPECTION.read().name.clone()}
                for section in INSPECTION.read().template.path(current()).into_iter().flatten() {
                    " > "
                    b { "{section}" }
                }
            }
        }

        div { class: "flex h-screen w-full bg-gray-100",
            Sidebar { current }
            Workspace { current }
        }
    }
}
