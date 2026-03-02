use leptos::prelude::*;
use lucide_leptos::{
    ArrowRight, CircleAlert, FileLock, FilePlus, FolderDown, LoaderCircle, UserRound,
};
use wasm_bindgen_futures::spawn_local;

use super::api::{sign_document, SignData};
use super::components::{
    drop_zone::DropZone, result_card::ResultCard, steps_flow::StepsFlow, steps_modal::StepsModal,
};

#[derive(Clone)]
enum SignState {
    Idle,
    Loading,
    Success(SignData),
    Error(String),
}

#[component]
pub fn SignPage() -> impl IntoView {
    let file = RwSignal::new(None::<web_sys::File>);
    let author = RwSignal::new(String::new());
    let state = RwSignal::new(SignState::Idle);
    let show_modal = RwSignal::new(false);

    let on_reset = move || {
        file.set(None);
        author.set(String::new());
        state.set(SignState::Idle);
    };

    let on_clear = Callback::new(move |_| {
        file.set(None);
        state.set(SignState::Idle);
    });

    let on_submit = move |_| {
        let Some(f) = file.get() else {
            state.set(SignState::Error("Please select a file first".to_string()));
            return;
        };
        let auth = author.get();
        if auth.trim().is_empty() {
            state.set(SignState::Error("Author name is required".to_string()));
            return;
        }
        state.set(SignState::Loading);
        spawn_local(async move {
            match sign_document(f, auth).await {
                Ok(data) => state.set(SignState::Success(data)),
                Err(e) => state.set(SignState::Error(e)),
            }
        });
    };

    view! {
        // -- modal
        {move || show_modal.get().then(|| view! {
            <StepsModal on_close=Callback::new(move |_| show_modal.set(false)) />
        })}

        <div class="max-w-2xl mx-auto px-4 py-12">

            // -- header
            <div class="mb-10">
                <h1 class="text-3xl font-display font-semibold text-primary-600 mb-4">
                    "Sign a Document"
                </h1>
                <p class="text-gray-500 leading-relaxed text-sm">
                    "Upload any file to embed an invisible cryptographic signature. "
                    "The signed file will be stored securely and registered in the audit registry."
                </p>
            </div>

            // -- form card
            {move || {
                if matches!(state.get(), SignState::Success(_)) {
                    view! { <div></div> }.into_any()
                } else {
                    view! {
                        <div class="card p-8 flex flex-col gap-6">

                            <div>
                                <label class="block text-sm font-semibold text-navy mb-2">
                                    "File"
                                </label>
                                <DropZone file=file on_clear=on_clear />
                                <div class="mt-3">
                                    <StepsFlow
                                        on_show_more=Callback::new(move |_| show_modal.set(true))
                                    />
                                </div>
                            </div>

                            <div>
                                <label class="block text-sm font-semibold text-navy mb-2">
                                    "Author"
                                </label>
                                <div class="relative">
                                    <div class="absolute left-3 top-1/2 -translate-y-1/2">
                                        <UserRound size=16 color="#9ca3af" />
                                    </div>
                                    <input
                                        type="text"
                                        placeholder="Your name or identifier"
                                        class="w-full pl-9 pr-4 py-3 text-sm border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-300 focus:border-primary-500 transition-all duration-200 bg-white"
                                        on:input=move |ev| author.set(event_target_value(&ev))
                                        prop:value=move || author.get()
                                    />
                                </div>
                            </div>

                            {move || {
                                if let SignState::Error(e) = state.get() {
                                    view! {
                                        <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                            <CircleAlert size=18 color="#dc2626" />
                                            {e}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}

                            {move || {
                                let loading = matches!(state.get(), SignState::Loading);
                                view! {
                                    <button
                                        class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg hover:shadow-primary-500/20 transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                        on:click=on_submit
                                        disabled=loading
                                    >
                                        {if loading {
                                            view! {
                                                <span class="animate-spin">
                                                    <LoaderCircle size=20 color="#ffffff" />
                                                </span>
                                                "Signing..."
                                            }.into_any()
                                        } else {
                                            view! {
                                                <FileLock size=20 color="#ffffff" />
                                                "Sign Document"
                                                <ArrowRight size=18 color="#ffffff" />
                                            }.into_any()
                                        }}
                                    </button>
                                }
                            }}
                        </div>
                    }.into_any()
                }
            }}

            // -- result card + action buttons
            {move || {
                if let SignState::Success(data) = state.get() {
                    // -- clona los campos necesarios ANTES de mover data a ResultCard
                    let doc_id   = data.document_id.clone();
                    let filename = data.filename.clone();
                    tracing::debug!(doc_id = %doc_id, filename = %filename, "success state — ready for download");

                    view! {
                        <div class="flex flex-col gap-4">
                            <ResultCard data=data />

                            <div class="grid grid-cols-2 gap-3">

                                // -- sign other
                                <button
                                    class="inline-flex items-center justify-center gap-2 px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 hover:border-primary-600 transform hover:scale-[1.02] transition-all duration-300"
                                    on:click=move |_| on_reset()
                                >
                                    <FilePlus size=18 color="#d20f39" />
                                    "Sign Other"
                                </button>

                                // -- download & close
                                <button
                                    class="inline-flex items-center justify-center gap-2 px-5 py-3 text-sm font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg hover:shadow-primary-500/20 transform hover:scale-[1.02] transition-all duration-300"
                                    on:click=move |_| {
                                        #[cfg(feature = "hydrate")]
                                        {
                                            use leptos::wasm_bindgen::JsCast;
                                            use crate::features::sign::api::download_url;
                                            let url = download_url(&doc_id);
                                            if let Some(window) = web_sys::window() {
                                                if let Some(document) = window.document() {
                                                    if let Ok(a) = document.create_element("a") {
                                                        let a = a.unchecked_into::<web_sys::HtmlAnchorElement>();
                                                        a.set_href(&url);
                                                        a.set_download(&format!("signed_{}", filename));
                                                        a.set_target("_blank");
                                                        let body = document.body().unwrap();
                                                        let _ = body.append_child(&a);
                                                        a.click();
                                                        let _ = body.remove_child(&a);
                                                    }
                                                }
                                            }
                                        }
                                        on_reset();
                                    }
                                >
                                    <FolderDown size=18 color="#ffffff" />
                                    "Download & Close"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
