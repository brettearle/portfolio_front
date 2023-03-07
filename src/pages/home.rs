use crate::components::{about_section, contact, projects_section, title};
use leptos::{component, view, IntoView, Scope};

use about_section::*;
use contact::*;
use projects_section::*;
use title::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
        <Title title={"Brett Earle".to_string()} />
        <About />
        <Projects />
        <Contact />
    }
}
