#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    launch(|| {
        rsx! {
            style { {include_str!("../assets/main.css")} }
            Router::<Route> {}
        }
    })
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)] // wrap the entire app in a header
        #[route("/")]
        Home {},

        #[route("/resume")]
        Resume {},
        
        #[route("/toolkit")]
        Toolkit{},

}

#[component]
fn Header() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home {}, class: "nav-btn", "Home" }
            Link { to: Route::Resume {}, class: "nav-btn", "Resume" }
            Link { to: Route::Toolkit {}, class: "nav-btn", "Toolkit" }
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
fn Toolkit() -> Element {
    rsx!(
            h1 { "Toolkit"}
        p {"Here are some of the tools i have worked with in any capacity"}
    )
}
