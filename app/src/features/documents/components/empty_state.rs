use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::{ArrowRight, FileLock, FileStack};

#[component]
pub fn EmptyState() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 gap-6 text-center">
            <div class="p-6 bg-gray-50 rounded-2xl">
                <FileStack size=48 color="#d1d5db" />
            </div>
            <div>
                <h3 class="text-lg font-display font-bold text-navy mb-2">
                    "No documents yet"
                </h3>
                <p class="text-sm text-gray-400 max-w-xs">
                    "Sign your first document to start building the audit registry."
                </p>
            </div>
            <A
                href="/sign"
                attr:class="inline-flex items-center gap-2 px-6 py-3 text-sm font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg hover:shadow-primary-500/20 transform hover:scale-[1.02] transition-all duration-300"
            >
                <FileLock size=16 color="#ffffff" />
                "Sign a Document"
                <ArrowRight size=16 color="#ffffff" />
            </A>
        </div>
    }
}
