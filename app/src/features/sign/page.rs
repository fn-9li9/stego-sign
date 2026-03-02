use leptos::prelude::*;

#[component]
pub fn SignPage() -> impl IntoView {
    view! {
        <div class="py-10 max-w-2xl mx-auto">
            <span class="section-label">"Document Signing"</span>
            <h1 class="section-title">"Sign a Document"</h1>
            <p class="text-gray-500 mb-8">
                "Upload a file to embed an invisible cryptographic signature. "
                "The signed file will be stored securely and registered in the audit registry."
            </p>
            // -- upload form goes here
            <div class="card p-8 text-center text-gray-400">
                "Upload form coming soon"
            </div>
        </div>
    }
}
