use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use lucide_leptos::{ArrowRight, CircleAlert, FileSearch, Hash, LoaderCircle, RotateCcw, Upload};

use super::api::{CodeVerifyData, VerifyData};
use super::components::{
    code_input::{format_code, CodeInput},
    code_result::CodeResultCard,
    drop_zone::VerifyDropZone,
    result_card::VerifyResultCard,
    steps_flow::VerifyStepsFlow,
    steps_modal::VerifyStepsModal,
};

#[derive(Clone, PartialEq)]
enum Tab {
    Upload,
    Code,
}

#[allow(dead_code)]
#[derive(Clone)]
enum VerifyState {
    Idle,
    Loading,
    Success(VerifyData),
    CodeSuccess(CodeVerifyData),
    Error(String),
}

#[component]
pub fn VerifyPage() -> impl IntoView {
    let file = RwSignal::new(None::<web_sys::File>);
    let state = RwSignal::new(VerifyState::Idle);
    let show_modal = RwSignal::new(false);
    let active_tab = RwSignal::new(Tab::Upload);
    let code_raw = RwSignal::new(String::new());

    // -- si viene con ?code= en la URL
    let query = use_query_map();
    let initial_code =
        query.with(|q: &leptos_router::params::ParamsMap| q.get("code").unwrap_or_default());

    if !initial_code.is_empty() {
        let raw: String = initial_code
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_uppercase())
            .take(6)
            .collect();
        active_tab.set(Tab::Code);
        code_raw.set(raw.clone());

        #[cfg(feature = "hydrate")]
        {
            use super::api::verify_by_code;
            use wasm_bindgen_futures::spawn_local;
            let formatted = format_code(&raw);
            spawn_local(async move {
                state.set(VerifyState::Loading);
                match verify_by_code(formatted).await {
                    Ok(d) => state.set(VerifyState::CodeSuccess(d)),
                    Err(e) => state.set(VerifyState::Error(e)),
                }
            });
        }
    }

    let on_reset = move || {
        file.set(None);
        code_raw.set(String::new());
        state.set(VerifyState::Idle);
    };

    let on_clear = Callback::new(move |_| {
        file.set(None);
        state.set(VerifyState::Idle);
    });

    let on_submit_file = move |_| {
        let Some(f) = file.get() else {
            state.set(VerifyState::Error("Please select a file first".to_string()));
            return;
        };
        state.set(VerifyState::Loading);
        #[cfg(feature = "hydrate")]
        {
            use super::api::verify_document;
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                match verify_document(f).await {
                    Ok(d) => state.set(VerifyState::Success(d)),
                    Err(e) => state.set(VerifyState::Error(e)),
                }
            });
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = f;
    };

    let on_submit_code = move |_| {
        let raw = code_raw.get();
        let formatted = format_code(&raw);
        if formatted.len() != 7 {
            state.set(VerifyState::Error(
                "Please enter all 6 characters".to_string(),
            ));
            return;
        }
        state.set(VerifyState::Loading);
        #[cfg(feature = "hydrate")]
        {
            use super::api::verify_by_code;
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                match verify_by_code(formatted).await {
                    Ok(d) => state.set(VerifyState::CodeSuccess(d)),
                    Err(e) => state.set(VerifyState::Error(e)),
                }
            });
        }
    };

    view! {
        {move || show_modal.get().then(|| view! {
            <VerifyStepsModal on_close=Callback::new(move |_| show_modal.set(false)) />
        })}

        <div class="max-w-2xl mx-auto px-4 py-12">

            // -- header
            <div class="mb-10">
                <p class="text-sm font-semibold text-primary-600 mb-1">"Document Verification"</p>
                <h1 class="text-3xl font-display font-semibold text-primary-600 mb-3">
                    "Verify a Document"
                </h1>
                <p class="text-gray-500 text-sm leading-relaxed">
                    "Upload a signed file or enter a verification code to check authenticity."
                </p>
            </div>

            // -- resultados
            {move || match state.get() {
                VerifyState::Success(data) => view! {
                    <div class="flex flex-col gap-4">
                        <VerifyResultCard data=data />
                        <button
                            class="inline-flex items-center justify-center gap-2 w-full px-5 py-3 text-sm font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-xl hover:bg-primary-50 transform hover:scale-[1.02] transition-all duration-300"
                            on:click=move |_| on_reset()
                        >
                            <RotateCcw size=18 color="#d20f39" />
                            "Verify Another"
                        </button>
                    </div>
                }.into_any(),

                VerifyState::CodeSuccess(data) => view! {
                    <CodeResultCard
                        data=data
                        on_reset=Callback::new(move |_| on_reset())
                    />
                }.into_any(),

                _ => view! {
                    <div class="card p-8 flex flex-col gap-6">

                        // -- tabs
                        <div class="flex gap-1 p-1 bg-gray-100 rounded-xl w-fit">
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Upload { "bg-white text-primary-600 shadow-sm" } else { "text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| { active_tab.set(Tab::Upload); state.set(VerifyState::Idle); }
                            >
                                <Upload size=15 color=if active_tab.get() == Tab::Upload { "#d20f39" } else { "#9ca3af" } />
                                "Upload File"
                            </button>
                            <button
                                class=move || format!(
                                    "inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-lg transition-all duration-200 {}",
                                    if active_tab.get() == Tab::Code { "bg-white text-primary-600 shadow-sm" } else { "text-gray-500 hover:text-gray-700" }
                                )
                                on:click=move |_| { active_tab.set(Tab::Code); state.set(VerifyState::Idle); }
                            >
                                <Hash size=15 color=if active_tab.get() == Tab::Code { "#d20f39" } else { "#9ca3af" } />
                                "Verify Code"
                            </button>
                        </div>

                        // -- tab content
                        {move || match active_tab.get() {

                            Tab::Upload => view! {
                                <div class="flex flex-col gap-6">
                                    <div>
                                        <label class="block text-sm font-semibold text-navy mb-2">
                                            "Signed File"
                                        </label>
                                        <VerifyDropZone file=file on_clear=on_clear />
                                        <div class="mt-3">
                                            <VerifyStepsFlow
                                                on_show_more=Callback::new(move |_| show_modal.set(true))
                                            />
                                        </div>
                                    </div>

                                    {move || if let VerifyState::Error(e) = state.get() {
                                        view! {
                                            <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                                <CircleAlert size=18 color="#dc2626" />
                                                {e}
                                            </div>
                                        }.into_any()
                                    } else { view! { <div></div> }.into_any() }}

                                    {move || {
                                        let loading = matches!(state.get(), VerifyState::Loading);
                                        view! {
                                            <button
                                                class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                                on:click=on_submit_file
                                                disabled=loading
                                            >
                                                {if loading {
                                                    view! {
                                                        <span class="animate-spin"><LoaderCircle size=20 color="#ffffff" /></span>
                                                        "Analyzing..."
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <FileSearch size=20 color="#ffffff" />
                                                        "Verify Document"
                                                        <ArrowRight size=18 color="#ffffff" />
                                                    }.into_any()
                                                }}
                                            </button>
                                        }
                                    }}
                                </div>
                            }.into_any(),

                            Tab::Code => view! {
                                <div class="flex flex-col gap-6">
                                    <div>
                                        <label class="block text-sm font-semibold text-navy mb-1">
                                            "Verification Code"
                                        </label>
                                        <p class="text-xs text-gray-400 mb-4">
                                            "Found on the QR embedded in the signed document. Format: "
                                            <span class="font-mono font-bold text-primary-500">"ABC-123"</span>
                                        </p>
                                        <CodeInput value=code_raw />
                                    </div>

                                    {move || if let VerifyState::Error(e) = state.get() {
                                        view! {
                                            <div class="flex items-center gap-3 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
                                                <CircleAlert size=18 color="#dc2626" />
                                                {e}
                                            </div>
                                        }.into_any()
                                    } else { view! { <div></div> }.into_any() }}

                                    {move || {
                                        let loading = matches!(state.get(), VerifyState::Loading);
                                        view! {
                                            <button
                                                class="inline-flex items-center justify-center gap-3 w-full px-6 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-xl hover:from-primary-600 hover:to-primary-700 hover:shadow-lg transform hover:scale-[1.01] transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none"
                                                on:click=on_submit_code
                                                disabled=loading
                                            >
                                                {if loading {
                                                    view! {
                                                        <span class="animate-spin"><LoaderCircle size=20 color="#ffffff" /></span>
                                                        "Checking..."
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <Hash size=20 color="#ffffff" />
                                                        "Verify Code"
                                                        <ArrowRight size=18 color="#ffffff" />
                                                    }.into_any()
                                                }}
                                            </button>
                                        }
                                    }}
                                </div>
                            }.into_any(),
                        }}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
