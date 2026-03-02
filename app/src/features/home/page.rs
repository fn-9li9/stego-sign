use super::api::Stats;
use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::{
    Activity, ArrowRight, FileCheck, FileLock, FileSearch, HardDrive, ShieldCheck,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let stats = RwSignal::new(Stats::default());

    #[cfg(feature = "hydrate")]
    {
        use super::api::fetch_stats;
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            match fetch_stats().await {
                Ok(s) => stats.set(s),
                Err(e) => tracing::warn!(error = %e, "failed to fetch stats"),
            }
        });
    }

    view! {
        <div class="relative min-h-screen flex flex-col">

            // -- hero
            <section class="flex-1 flex flex-col items-center justify-center text-center px-4 py-24">
                <div class="inline-flex items-center gap-2 bg-primary-50 border border-primary-200 text-primary-600 text-sm font-semibold px-4 py-2 rounded-full mb-8">
                    <span class="w-2 h-2 bg-primary-500 rounded-full animate-pulse"></span>
                    "Document Integrity System"
                </div>

                <h1 class="text-5xl md:text-7xl font-display font-bold text-navy mb-6 leading-tight">
                    "Protect Your"
                    <br/>
                    <span class="bg-gradient-to-r from-primary-500 via-primary-600 to-accent bg-clip-text text-transparent">
                        "Documents"
                    </span>
                </h1>

                <p class="text-lg md:text-xl text-gray-500 max-w-2xl mb-12 leading-relaxed">
                    "Embed cryptographic signatures using steganography. "
                    "Detect tampering instantly. "
                    "Verify authenticity with forensic-grade analysis."
                </p>

                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    <A
                        href="/sign"
                        attr:class="group inline-flex items-center gap-3 px-8 py-4 text-base font-semibold text-white bg-gradient-to-r from-primary-500 to-primary-600 rounded-full hover:from-primary-600 hover:to-primary-700 transform hover:scale-105 hover:shadow-xl hover:shadow-primary-500/30 transition-all duration-300 shadow-lg"
                    >
                        <FileLock size=20 color="#ffffff" />
                        "Sign a Document"
                        <span class="transform group-hover:translate-x-1 transition-transform duration-300">
                            <ArrowRight size=18 color="#ffffff" />
                        </span>
                    </A>
                    <A
                        href="/verify"
                        attr:class="group inline-flex items-center gap-3 px-8 py-4 text-base font-semibold text-primary-600 bg-white border-2 border-primary-500 rounded-full hover:bg-primary-50 hover:border-primary-600 hover:shadow-lg transform hover:scale-105 transition-all duration-300"
                    >
                        <FileSearch size=20 color="#d20f39" />
                        "Verify Integrity"
                        <span class="transform group-hover:translate-x-1 transition-transform duration-300">
                            <ArrowRight size=18 color="#d20f39" />
                        </span>
                    </A>
                </div>
            </section>

            // -- features
            <section class="py-20 px-4 bg-gray-50">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <span class="section-label justify-center">"How it works"</span>
                        <h2 class="section-title">"Three steps to trust"</h2>
                        <p class="section-subtitle">
                            "A reproducible SRE-grade pipeline for document authenticity"
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <FeatureCard
                            step="01"
                            title="Sign"
                            description="Upload any file. A SHA-256 hash and Ed25519 signature are embedded invisibly using steganography."
                            href="/sign"
                            cta="Sign a document"
                        >
                            <FileLock size=32 color="#d20f39" />
                        </FeatureCard>
                        <FeatureCard
                            step="02"
                            title="Verify"
                            description="Upload a signed file. The pipeline extracts the payload, verifies the signature and cross-checks the registry."
                            href="/verify"
                            cta="Verify a document"
                        >
                            <FileSearch size=32 color="#d20f39" />
                        </FeatureCard>
                        <FeatureCard
                            step="03"
                            title="Audit"
                            description="Every verification is logged. Access the full forensic history of any document at any time."
                            href="/documents"
                            cta="View documents"
                        >
                            <FileCheck size=32 color="#d20f39" />
                        </FeatureCard>
                    </div>
                </div>
            </section>

            // -- stats reactivos
            <section class="py-16 px-4 bg-white border-t border-gray-100">
                <div class="max-w-6xl mx-auto">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">

                        <StatusStat
                            label="Documents Signed"
                            value=Signal::derive(move || {
                                let v = stats.get().documents_signed;
                                if v == 0 { "—".to_string() } else { v.to_string() }
                            })
                        >
                            <FileLock size=24 color="#d20f39" />
                        </StatusStat>

                        <StatusStat
                            label="Verifications"
                            value=Signal::derive(move || {
                                let v = stats.get().verifications;
                                if v == 0 { "—".to_string() } else { v.to_string() }
                            })
                        >
                            <ShieldCheck size=24 color="#d20f39" />
                        </StatusStat>

                        <StatusStat
                            label="Tampered Detected"
                            value=Signal::derive(move || {
                                let v = stats.get().tampered;
                                if v == 0 { "—".to_string() } else { v.to_string() }
                            })
                        >
                            <Activity size=24 color="#f59e0b" />
                        </StatusStat>

                        <StatusStat
                            label="Objects in Storage"
                            value=Signal::derive(move || {
                                let v = stats.get().objects;
                                if v == 0 { "—".to_string() } else { v.to_string() }
                            })
                        >
                            <HardDrive size=24 color="#1e293b" />
                        </StatusStat>

                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(
    step: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
    cta: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card card-hover p-8 flex flex-col gap-4 group">
            <div class="flex items-center justify-between">
                <div class="p-3 bg-primary-50 rounded-xl group-hover:bg-primary-100 transition-colors duration-300">
                    {children()}
                </div>
                <span class="text-2xl font-bold text-gray-200 font-display">{step}</span>
            </div>
            <h3 class="text-xl font-display font-bold text-navy">{title}</h3>
            <p class="text-gray-500 text-sm leading-relaxed flex-1">{description}</p>
            <a
                href=href
                class="inline-flex items-center gap-2 text-sm font-semibold text-primary-500 hover:text-primary-600 transition-colors group/link"
            >
                {cta}
                <span class="group-hover/link:translate-x-1 transition-transform duration-200">
                    <ArrowRight size=16 color="#d20f39" />
                </span>
            </a>
        </div>
    }
}

#[component]
fn StatusStat(label: &'static str, value: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2">
            <div class="p-3 bg-gray-50 rounded-xl">
                {children()}
            </div>
            <span class="text-3xl font-display font-bold text-navy">{move || value.get()}</span>
            <span class="text-sm text-gray-500">{label}</span>
        </div>
    }
}
