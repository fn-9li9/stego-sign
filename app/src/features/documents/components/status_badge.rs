use leptos::prelude::*;
use lucide_leptos::{CircleAlert, CircleCheck, CircleQuestionMark, CircleX};

#[component]
pub fn StatusBadge(status: String) -> impl IntoView {
    let (icon, text_class, bg_class, border_class, label) = match status.as_str() {
        "VALID" => (
            view! { <CircleCheck size=13 color="#16a34a" /> }.into_any(),
            "text-green-700",
            "bg-green-50",
            "border-green-200",
            "VALID",
        ),
        "TAMPERED" => (
            view! { <CircleX size=13 color="#dc2626" /> }.into_any(),
            "text-red-700",
            "bg-red-50",
            "border-red-200",
            "TAMPERED",
        ),
        "UNREGISTERED" => (
            view! { <CircleAlert size=13 color="#d97706" /> }.into_any(),
            "text-yellow-700",
            "bg-yellow-50",
            "border-yellow-200",
            "UNREGISTERED",
        ),
        _ => (
            view! { <CircleQuestionMark size=13 color="#6b7280" /> }.into_any(),
            "text-gray-600",
            "bg-gray-50",
            "border-gray-200",
            "INVALID",
        ),
    };

    view! {
        <span class=format!(
            "inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-bold rounded-lg border {} {} {}",
            text_class, bg_class, border_class
        )>
            {icon}
            {label}
        </span>
    }
}
