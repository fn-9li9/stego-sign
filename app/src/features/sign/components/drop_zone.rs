use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use lucide_leptos::{FileText, Upload, X};

#[component]
pub fn DropZone(file: RwSignal<Option<web_sys::File>>, on_clear: Callback<()>) -> impl IntoView {
    let dragging = RwSignal::new(false);

    let on_change = move |ev: web_sys::Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(f) = files.get(0) {
                    file.set(Some(f));
                }
            }
        }
    };

    let on_dragover = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        dragging.set(true);
    };

    let on_dragleave = move |_: web_sys::DragEvent| {
        dragging.set(false);
    };

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        dragging.set(false);
        // -- data_transfer() disponible directamente en web_sys::DragEvent
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                if let Some(f) = files.get(0) {
                    file.set(Some(f));
                }
            }
        }
    };

    view! {
        <div>
            {move || match file.get() {
                None => view! {
                    <label
                        class=move || format!(
                            "flex flex-col items-center justify-center w-full h-48 border-2 border-dashed rounded-2xl cursor-pointer transition-all duration-300 {}",
                            if dragging.get() {
                                "border-primary-500 bg-primary-50 scale-[1.02]"
                            } else {
                                "border-gray-300 bg-gray-50 hover:border-primary-400 hover:bg-primary-50/50"
                            }
                        )
                        on:dragover=on_dragover
                        on:dragleave=on_dragleave
                        on:drop=on_drop
                    >
                        <div class="flex flex-col items-center gap-3 pointer-events-none">
                            <div class=move || format!(
                                "p-4 rounded-full transition-colors duration-300 {}",
                                if dragging.get() { "bg-primary-100" } else { "bg-gray-100" }
                            )>
                                {move || if dragging.get() {
                                    view! { <Upload size=28 color="#d20f39" /> }.into_any()
                                } else {
                                    view! { <Upload size=28 color="#9ca3af" /> }.into_any()
                                }}
                            </div>
                            <div class="text-center">
                                <p class="text-sm font-semibold text-gray-700">
                                    "Drop your file here"
                                </p>
                                <p class="text-xs text-gray-400 mt-1">
                                    "or click to browse — any file type supported"
                                </p>
                            </div>
                        </div>
                        <input
                            type="file"
                            class="hidden"
                            on:change=on_change
                        />
                    </label>
                }.into_any(),

                Some(f) => view! {
                    <div class="flex items-center gap-4 p-4 bg-primary-50 border border-primary-200 rounded-2xl">
                        <div class="p-3 bg-white rounded-xl shadow-sm">
                            <FileText size=24 color="#d20f39" />
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-semibold text-navy truncate">
                                {f.name()}
                            </p>
                            <p class="text-xs text-gray-400 mt-0.5">
                                {format!("{:.1} KB", f.size() / 1024.0)}
                            </p>
                        </div>
                        <button
                            class="p-2 text-gray-400 hover:text-primary-500 hover:bg-white rounded-lg transition-all duration-200"
                            on:click=move |_| on_clear.run(())
                        >
                            <X size=18 color="#9ca3af" />
                        </button>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
