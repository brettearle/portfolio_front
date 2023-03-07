use leptos::{component, create_signal, event_target_value, view, IntoView, Scope};

#[component]
pub fn Contact(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "email".to_string());

    let update_email = move |ev| set_email(event_target_value(&ev));

    view! {cx,
        <section>
            <h2>"Contact"</h2>
            <input type="text" prop:value=email on:input=update_email/>
            <p>"Email is " {email}</p>
        </section>
    }
}
