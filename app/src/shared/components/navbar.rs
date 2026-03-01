use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 bg-slate-900 border-b border-slate-700 px-6 py-4">
            <div class="max-w-6xl mx-auto flex items-center justify-between">
                <span class="text-indigo-400 font-bold text-xl">"StegoSign"</span>
                <div class="flex gap-6">
                    <A href="/"          attr:class="text-slate-300 hover:text-indigo-400 transition-colors">"Sign"</A>
                    <A href="/verify"    attr:class="text-slate-300 hover:text-indigo-400 transition-colors">"Verify"</A>
                    <A href="/documents" attr:class="text-slate-300 hover:text-indigo-400 transition-colors">"Documents"</A>
                </div>
            </div>
        </nav>
    }
}
