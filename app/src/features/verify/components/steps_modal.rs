use leptos::prelude::*;
use lucide_leptos::{
    ArrowDown, CircleCheck, DatabaseZap, FileSearch, Hash, ScanLine, ShieldCheck, Upload, X,
};

#[component]
pub fn VerifyStepsModal(on_close: Callback<()>) -> impl IntoView {
    view! {
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4"
            style="background-color: rgba(15,23,42,0.6); backdrop-filter: blur(4px);"
            on:click=move |_| on_close.run(())
        >
            <div
                class="bg-white rounded-2xl shadow-2xl w-full max-w-lg max-h-[90vh] overflow-y-auto"
                on:click=|ev| ev.stop_propagation()
            >
                // -- header
                <div class="flex items-center justify-between px-6 py-5 border-b border-gray-100">
                    <div class="flex items-center gap-2">
                        <ShieldCheck size=20 color="#d20f39" />
                        <h2 class="font-display font-bold text-navy text-lg">"How verification works"</h2>
                    </div>
                    <button
                        class="p-2 text-gray-400 hover:text-primary-500 hover:bg-primary-50 rounded-xl transition-all"
                        on:click=move |_| on_close.run(())
                    >
                        <X size=18 color="#9ca3af" />
                    </button>
                </div>

                // -- steps
                <div class="px-6 py-5 flex flex-col gap-4">
                    <ModalStep
                        number="01"
                        icon_slot=view! { <Upload size=20 color="#d20f39" /> }.into_any()
                        title="File Upload"
                        description="The signed file is received via multipart form. It can be any file that was previously signed with this system."
                    />
                    <Divider />
                    <ModalStep
                        number="02"
                        icon_slot=view! { <FileSearch size=20 color="#d20f39" /> }.into_any()
                        title="Payload Extraction"
                        description="The pipeline scans the file for the magic delimiter >>STEGO::PAYLOAD<< ... >>STEGO::END<<. If not found, the file is classified as INVALID immediately."
                    />
                    <Divider />
                    <ModalStep
                        number="03"
                        icon_slot=view! { <Hash size=20 color="#d20f39" /> }.into_any()
                        title="Hash Recomputation"
                        description="The SHA-256 hash of the current file bytes (without the payload) is recomputed and compared against the hash stored in the embedded payload."
                    />
                    <Divider />
                    <ModalStep
                        number="04"
                        icon_slot=view! { <ScanLine size=20 color="#d20f39" /> }.into_any()
                        title="Signature Verification"
                        description="The Ed25519 public key is used to verify the embedded signature against the original hash. A mismatch means the file was tampered with."
                    />
                    <Divider />
                    <ModalStep
                        number="05"
                        icon_slot=view! { <DatabaseZap size=20 color="#d20f39" /> }.into_any()
                        title="Registry Cross-check"
                        description="The document ID and hash are cross-checked against the PostgreSQL registry. A valid signature with no registry entry is classified as UNREGISTERED."
                    />
                    <Divider />
                    <ModalStep
                        number="06"
                        icon_slot=view! { <CircleCheck size=20 color="#16a34a" /> }.into_any()
                        title="Verdict"
                        description="Final classification: VALID (all checks pass), TAMPERED (hash or signature mismatch), UNREGISTERED (valid signature but not in registry), or INVALID (no payload found)."
                    />

                    <div class="mt-2 p-4 bg-primary-50 border border-primary-100 rounded-xl flex items-start gap-3">
                        <CircleCheck size=16 color="#d20f39" />
                        <p class="text-xs text-primary-700 leading-relaxed">
                            "Every verification attempt is recorded in the audit log with its verdict, timestamp and file hash — providing a complete forensic trail."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ModalStep(
    number: &'static str,
    icon_slot: AnyView,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-4">
            <div class="flex flex-col items-center gap-1 shrink-0">
                <div class="p-2.5 bg-primary-50 rounded-xl border border-primary-100">
                    {icon_slot}
                </div>
                <span class="text-xs font-bold text-gray-200">{number}</span>
            </div>
            <div class="pt-1">
                <h4 class="font-display font-semibold text-navy text-sm mb-1">{title}</h4>
                <p class="text-xs text-gray-500 leading-relaxed">{description}</p>
            </div>
        </div>
    }
}

#[component]
fn Divider() -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 pl-5">
            <ArrowDown size=14 color="#e5e7eb" />
        </div>
    }
}
