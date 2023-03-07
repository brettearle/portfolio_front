use leptos::{component, view, IntoView, Scope};

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! {cx,
        <section>
            <h2>"About"</h2>
            <p>"A software engineer that loves to build solutions and solve complex issues within a collaborative environment. I currently love working within the Javascript ecosystem employing technologies that include React and Node. The next tech I will be self-teaching is typescript."</p>
            <p>"After leaving my mates and team in the military post-deployment, I had an opportunity to continue to learn. During this time as a service technician, I loved helping the team learn and being a mentor. This led to a role as the development lead. As part of this role I designed and implemented an in-house training program using code. I spent time building an education platform aimed at in-house training programs in the trades. This unlocked a drive to employ these skills as a Software Engineer with global reach."</p>
        </section>
    }
}
