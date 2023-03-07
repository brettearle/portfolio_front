pub mod components;
pub mod pages;
use leptos::{component, view, IntoView, Scope};
use leptos_router::*;
use pages::home::{Home, HomeProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Router>
        <nav>
          <A href="" >"Home"</A>
        </nav>
        <main>
        <Routes>
            <Route path="/" view=move |cx| view! {cx, <Home/>} />
        </Routes>
        </main>
    </Router>
    }
}
