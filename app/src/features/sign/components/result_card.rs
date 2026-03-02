use super::super::api::{download_url, SignData};
use leptos::prelude::*;
use lucide_leptos::{CircleCheck, Copy, Download, Hash, KeyRound};

#[component]
pub fn ResultCard(data: SignData) -> impl IntoView {
    let copied = RwSignal::new(false);
    let hash = data.hash.clone();

    let copy_hash = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use leptos::wasm_bindgen::JsCast;
            use wasm_bindgen_futures::spawn_local;
            let h = hash.clone();
            spawn_local(async move {
                // -- accede al clipboard via js directamente
                let window = web_sys::window().unwrap();
                let nav: web_sys::Navigator = window.navigator();
                let clipboard = nav.clipboard();
                let _ = clipboard.write_text(&h);
                copied.set(true);
                gloo_timers::future::TimeoutFuture::new(2000).await;
                copied.set(false);
            });
        }
    };

    view! {
        <div class="flex flex-col gap-6 p-6 bg-white border border-green-200 rounded-2xl shadow-sm">

            // -- success header
            <div class="flex items-center gap-3">
                <div class="p-2 bg-green-50 rounded-xl">
                    <CircleCheck size=24 color="#16a34a" />
                </div>
                <div>
                    <h3 class="font-display font-bold text-navy">"Document Signed"</h3>
                    <p class="text-xs text-gray-400">"Signature embedded and registered"</p>
                </div>
            </div>

            // -- metadata
            <div class="flex flex-col gap-3">
                <MetaRow icon_color="#7287fd" label="Filename"    value=data.filename.clone() />
                <MetaRow icon_color="#7287fd" label="Author"      value=data.author.clone() />
                <MetaRow icon_color="#7287fd" label="Document ID" value=data.document_id.clone() />

                // -- hash con copy
                <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-xl">
                    <div class="mt-0.5 shrink-0">
                        <Hash size=16 color="#9ca3af" />
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-xs text-gray-400 mb-1">"SHA-256 Hash"</p>
                        <p class="text-xs font-mono text-gray-600 break-all">{data.hash.clone()}</p>
                    </div>
                    <button
                        class="shrink-0 p-1.5 text-gray-400 hover:text-primary-500 hover:bg-primary-50 rounded-lg transition-all"
                        on:click=copy_hash
                        title="Copy hash"
                    >
                        <Copy size=14 color="#9ca3af" />
                    </button>
                </div>

                {move || copied.get().then(|| view! {
                    <p class="text-xs text-green-600 font-medium text-center animate-pulse">
                        "Hash copied to clipboard!"
                    </p>
                })}
            </div>

            // -- download
            <a
                target="_blank"
                rel="noopener noreferrer"
                href=download_url(&data.document_id)
                download=format!("signed_{}", data.filename)
                class="inline-flex items-center justify-center gap-2 w-full px-6 py-3 text-sm font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg hover:shadow-primary-500/20 transform hover:scale-[1.02] transition-all duration-300"
            >
                <Download size=18 color="#ffffff" />
                "Download Signed File"
            </a>
        </div>
    }
}

#[component]
fn MetaRow(icon_color: &'static str, label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-xl">
            <div class="shrink-0">
                <KeyRound size=16 color=icon_color />
            </div>
            <div class="min-w-0">
                <p class="text-xs text-gray-400">{label}</p>
                <p class="text-sm font-medium text-navy truncate">{value}</p>
            </div>
        </div>
    }
}
