use leptos::prelude::*;

#[component]
pub fn DocumentsPage() -> impl IntoView {
    view! {
        <div class="py-10">
            <h1 class="text-3xl font-bold text-primary mb-2 text-primary-50">"Documents"</h1>
            <p class="text-slate-400">"All signed documents and their audit history."</p>
        </div>
    }
}
