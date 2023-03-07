mod components;
use components::title::{Title, TitleProps};
use leptos::{component, view, IntoView, Scope};
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Router>
        <nav>
          <A href="" >"Home"</A>
          <A href="/test" >"Test"</A>
        </nav>
        <main>
        <Routes>
            <Route path="/" view=move |cx| view! {cx, <Title title={"Brett Earle".to_string()}/>} />
            <Route path="/test" view=move |cx| view! {cx, <Title title={"test".to_string()}/>} />
        </Routes>
        </main>
    </Router>
    }
}
