mod components;
use components::landing_page::{Title, TitleProps};
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let full_name = "Brett Earle".to_string();

    view! { cx,
        <div>
            <Title title={full_name}/>
        </div>
    }
}
