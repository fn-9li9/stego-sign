use leptos::prelude::*;
use lucide_leptos::{
    ArrowRight, CircleCheck, DatabaseZap, HardDriveUpload, Hash, Info, ScanLine, Upload,
};

#[component]
pub fn StepsFlow(on_show_more: Callback<()>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <div class="flex items-center gap-1.5 flex-wrap">
                <Step icon_slot=view! { <Upload size=13 color="#d20f39" /> }.into_any()         label="Upload" />
                <ArrowRight size=11 color="#d1d5db" />
                <Step icon_slot=view! { <Hash size=13 color="#d20f39" /> }.into_any()           label="SHA-256" />
                <ArrowRight size=11 color="#d1d5db" />
                <Step icon_slot=view! { <ScanLine size=13 color="#d20f39" /> }.into_any()       label="Ed25519" />
                <ArrowRight size=11 color="#d1d5db" />
                <Step icon_slot=view! { <HardDriveUpload size=13 color="#d20f39" /> }.into_any() label="Stego embed" />
                <ArrowRight size=11 color="#d1d5db" />
                <Step icon_slot=view! { <DatabaseZap size=13 color="#d20f39" /> }.into_any()    label="Registry" />
                <ArrowRight size=11 color="#d1d5db" />
                <Step icon_slot=view! { <CircleCheck size=13 color="#16a34a" /> }.into_any()    label="Signed" />

                <button
                    class="ml-1 p-1 rounded-full hover:bg-primary-50 transition-colors duration-200"
                    on:click=move |_| on_show_more.run(())
                    title="How it works"
                >
                    <Info size=13 color="#b00d30" />
                </button>
            </div>
        </div>
    }
}

#[component]
fn Step(icon_slot: AnyView, label: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-1 bg-gray-50 border border-gray-200 px-2 py-1 rounded-lg">
            {icon_slot}
            <span class="text-xs text-gray-500 font-medium">{label}</span>
        </div>
    }
}
