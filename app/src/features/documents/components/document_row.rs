use leptos::prelude::*;
use lucide_leptos::{
    CircleAlert, CircleCheck, CircleQuestionMark, CircleX, Clock, Download, FileLock, FileSearch,
    Hash,
};

use super::super::api::{download_url, SignedDoc, VerificationEntry};

// -- fila para documento firmado
#[component]
pub fn SignedDocRow(doc: SignedDoc) -> impl IntoView {
    let hash_short = if doc.hash_sha256.len() >= 64 {
        format!("{}…{}", &doc.hash_sha256[..8], &doc.hash_sha256[56..])
    } else {
        doc.hash_sha256.clone()
    };

    let signed_at = doc
        .signed_at
        .split('+')
        .next()
        .unwrap_or(&doc.signed_at)
        .split('.')
        .next()
        .unwrap_or(&doc.signed_at)
        .replace('T', " ")
        .to_string();

    let (status_icon, status_class) = match doc.status.as_str() {
        "VALID" => (
            view! { <CircleCheck size=14 color="#16a34a" /> }.into_any(),
            "bg-green-50 border-green-200 text-green-700",
        ),
        "TAMPERED" => (
            view! { <CircleX size=14 color="#dc2626" /> }.into_any(),
            "bg-red-50 border-red-200 text-red-700",
        ),
        _ => (
            view! { <CircleAlert size=14 color="#d97706" /> }.into_any(),
            "bg-yellow-50 border-yellow-200 text-yellow-700",
        ),
    };
    let status_label = doc.status.clone();

    view! {
        <div class="flex items-center gap-4 p-4 bg-white border border-gray-100 rounded-xl hover:border-primary-200 hover:shadow-sm transition-all duration-200 group">

            // -- icono
            <div class="p-2.5 bg-gray-50 rounded-xl shrink-0 group-hover:bg-primary-50 transition-colors">
                <FileLock size=18 color="#d20f39" />
            </div>

            // -- filename + hash
            <div class="flex-1 min-w-0">
                <p class="text-sm font-semibold text-navy truncate">{doc.filename.clone()}</p>
                <div class="flex items-center gap-1 mt-0.5">
                    <Hash size=11 color="#9ca3af" />
                    <p class="text-xs font-mono text-gray-400">{hash_short}</p>
                </div>
            </div>

            // -- author
            <div class="hidden md:block w-28 shrink-0">
                <p class="text-xs text-gray-400">"Author"</p>
                <p class="text-sm font-medium text-navy truncate">{doc.author}</p>
            </div>

            // -- signed at
            <div class="hidden lg:block w-36 shrink-0">
                <p class="text-xs text-gray-400">"Signed At"</p>
                <p class="text-xs text-gray-600 font-medium">{signed_at}</p>
            </div>

            // -- status badge
            <span class=format!(
                "shrink-0 inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-bold rounded-lg border {}",
                status_class
            )>
                {status_icon}
                {status_label}
            </span>

            // -- download
            <a
                href=download_url(&doc.id)
                download=format!("signed_{}", doc.filename)
                class="shrink-0 p-2 text-gray-400 hover:text-primary-500 hover:bg-primary-50 rounded-lg transition-all duration-200"
                title="Download signed file"
            >
                <Download size=16 color="#9ca3af" />
            </a>
        </div>
    }
}

// -- fila para entrada de verificacion
#[component]
pub fn VerificationRow(entry: VerificationEntry) -> impl IntoView {
    let checked_at = entry
        .checked_at
        .split('+')
        .next()
        .unwrap_or(&entry.checked_at)
        .split('.')
        .next()
        .unwrap_or(&entry.checked_at)
        .replace('T', " ")
        .to_string();

    let hash_short = entry.checked_hash.as_deref().map(|h| {
        if h.len() >= 64 {
            format!("{}…{}", &h[..8], &h[56..])
        } else {
            h.to_string()
        }
    });

    let (result_icon, result_class) = match entry.result.as_str() {
        "VALID" => (
            view! { <CircleCheck size=14 color="#16a34a" /> }.into_any(),
            "bg-green-50 border-green-200 text-green-700",
        ),
        "TAMPERED" => (
            view! { <CircleX size=14 color="#dc2626" /> }.into_any(),
            "bg-red-50 border-red-200 text-red-700",
        ),
        "UNREGISTERED" => (
            view! { <CircleAlert size=14 color="#d97706" /> }.into_any(),
            "bg-yellow-50 border-yellow-200 text-yellow-700",
        ),
        _ => (
            view! { <CircleQuestionMark size=14 color="#6b7280" /> }.into_any(),
            "bg-gray-50 border-gray-200 text-gray-600",
        ),
    };
    let result_label = entry.result.clone();

    view! {
        <div class="flex items-center gap-4 p-4 bg-white border border-gray-100 rounded-xl hover:border-primary-200 hover:shadow-sm transition-all duration-200 group">

            // -- icono
            <div class="p-2.5 bg-gray-50 rounded-xl shrink-0 group-hover:bg-primary-50 transition-colors">
                <FileSearch size=18 color="#9ca3af" />
            </div>

            // -- filename + hash
            <div class="flex-1 min-w-0">
                <p class="text-sm font-semibold text-navy truncate">
                    {entry.filename.unwrap_or_else(|| "unknown file".to_string())}
                </p>
                {hash_short.map(|h| view! {
                    <div class="flex items-center gap-1 mt-0.5">
                        <Hash size=11 color="#9ca3af" />
                        <p class="text-xs font-mono text-gray-400">{h}</p>
                    </div>
                })}
            </div>

            // -- documento vinculado
            <div class="hidden md:block w-28 shrink-0">
                <p class="text-xs text-gray-400">"Document ID"</p>
                <p class="text-xs font-mono text-gray-500 truncate">
                    {entry.document_id
                        .as_deref()
                        .map(|id| if id.len() >= 8 { format!("{}…", &id[..8]) } else { id.to_string() })
                        .unwrap_or_else(|| "—".to_string())}
                </p>
            </div>

            // -- fecha verificacion
            <div class="hidden lg:flex items-center gap-1.5 w-40 shrink-0">
                <Clock size=13 color="#9ca3af" />
                <p class="text-xs text-gray-500 font-medium">{checked_at}</p>
            </div>

            // -- resultado badge
            <span class=format!(
                "shrink-0 inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-bold rounded-lg border {}",
                result_class
            )>
                {result_icon}
                {result_label}
            </span>
        </div>
    }
}
