use leptos::{component, create_signal, event_target_value, view, IntoView, Scope};

#[component]
pub fn Contact(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "name".to_string());
    let (email, set_email) = create_signal(cx, "email".to_string());

    let update_name = move |ev| set_name(event_target_value(&ev));
    let update_email = move |ev| set_email(event_target_value(&ev));

    view! {cx,
        <section>
            <h2>"Contact"</h2>
            <input type="text" prop:value=name on:input=update_name/>
            <input type="text" prop:value=email on:input=update_email/>
            <p>"Email is " {email}</p>
            <p>"Name is " {name}</p>
        </section>
    }
}
