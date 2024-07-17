#![allow(non_snake_case)]
//! Example: G//! This example shows how to use the `Router` component to create a simple navigation system.
//! The more complex router example uses all of the router features, while this simple exmaple showcases
//! just the `Layout` and `Route` features.
//!
//! Layouts let you wrap chunks of your app with a component. This is useful for things like a footers, headers, etc.
//! Routes are enum variants with that match the name of a component in scope. This way you can create a new route
//! in your app simply by adding the variant to the enum and creating a new component with the same name. You can
//! override this of course.

use dioxus::prelude::*;
use manganis::*;

fn main() {
    launch(|| {
        rsx! {
            style { {include_str!("../assets/flat_router.css")} }
            Router::<Route> {}
        }
    })
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Footer)] // wrap the entire app in a footer
        #[route("/")]
        Home {},

        #[route("/resume")]
        Resume {},

        #[route("/play")]
        Play {},

        #[route("/settings")]
        Settings {},
}

#[component]
fn Footer() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home {}, class: "nav-btn", "Home" }
            Link { to: Route::Resume {}, class: "nav-btn", "Resume" }
            Link { to: Route::Play {}, class: "nav-btn", "Play" }
            Link { to: Route::Settings {}, class: "nav-btn", "Settings" }
        }
        div { id: "content",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        h1 { "Home" }
        p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
    )
}

#[component]
fn Resume() -> Element {
    const RESUME_PDF: &str = manganis::mg!(file(
        "https://writing.colostate.edu/guides/documents/resume/functionalSample.pdf"
    ));
    rsx!(
        h1 { "Resume" }
        embed {src:"{RESUME_PDF}",
    width: 650,
    height: 850}
    )
}

#[component]
fn Play() -> Element {
    rsx!(
        h1 { "Play" }
        p { "Always play with your full heart adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
    )
}

#[component]
fn Settings() -> Element {
    rsx!(
        h1 { "Settings" }
        p { "Settings are consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
    )
}
