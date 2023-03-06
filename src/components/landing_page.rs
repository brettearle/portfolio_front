use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Title(cx: Scope, title: String) -> impl IntoView {
    view! {cx,
        <h1>{title}</h1>
    }
}
