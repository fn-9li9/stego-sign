use leptos::prelude::*;

#[component]
pub fn SignPage() -> impl IntoView {
    view! {
        <div class="py-10">
            <h1 class="text-3xl font-bold text-primary mb-2">"Sign Document"</h1>
            <p class="text-slate-400">"Upload a file to embed a cryptographic signature."</p>
        </div>
    }
}
