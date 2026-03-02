use leptos::prelude::*;
use lucide_leptos::{
    ArrowDown, CircleCheck, DatabaseZap, FileSearch, HardDriveUpload, Hash, ScanLine, ShieldCheck,
    Upload, X,
};

#[component]
pub fn StepsModal(on_close: Callback<()>) -> impl IntoView {
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
                        <h2 class="font-display font-bold text-navy text-lg">"How signing works"</h2>
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
                        description="Your file is received via multipart form. Any format is supported — PDF, PNG, DOCX, binary, etc. The original bytes are preserved unchanged."
                    />
                    <Divider />
                    <ModalStep
                        number="02"
                        icon_slot=view! { <Hash size=20 color="#d20f39" /> }.into_any()
                        title="SHA-256 Hash"
                        description="A cryptographic hash of the original file bytes is computed using SHA-256. This 256-bit fingerprint uniquely identifies the file content."
                    />
                    <Divider />
                    <ModalStep
                        number="03"
                        icon_slot=view! { <ScanLine size=20 color="#d20f39" /> }.into_any()
                        title="Ed25519 Signature"
                        description="The SHA-256 hash is signed using an Ed25519 private key. This produces a 64-byte digital signature that proves authorship and file integrity."
                    />
                    <Divider />
                    <ModalStep
                        number="04"
                        icon_slot=view! { <HardDriveUpload size=20 color="#d20f39" /> }.into_any()
                        title="Steganographic Embedding"
                        description="A JSON payload containing the document ID, hash, signature and author is appended to the file using magic delimiters >>STEGO::PAYLOAD<< ... >>STEGO::END<<. The signed file looks identical to the original."
                    />
                    <Divider />
                    <ModalStep
                        number="05"
                        icon_slot=view! { <DatabaseZap size=20 color="#d20f39" /> }.into_any()
                        title="Registry & Storage"
                        description="The original and signed files are uploaded to MinIO. The document is registered in PostgreSQL with its hash, signature, author and MinIO object keys for full traceability."
                    />
                    <Divider />
                    <ModalStep
                        number="06"
                        icon_slot=view! { <FileSearch size=20 color="#16a34a" /> }.into_any()
                        title="Verification ready"
                        description="The signed file can now be verified at any time. The pipeline will extract the payload, recompute the hash, verify the Ed25519 signature and cross-check the registry — classifying the result as VALID, TAMPERED, UNREGISTERED or INVALID."
                    />

                    <div class="mt-2 p-4 bg-primary-50 border border-primary-100 rounded-xl flex items-start gap-3">
                        <CircleCheck size=16 color="#d20f39" />
                        <p class="text-xs text-primary-700 leading-relaxed">
                            "All operations are deterministic and reproducible. The entire pipeline runs in Docker and can be re-executed in any environment, satisfying SRE reproducibility principles."
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
