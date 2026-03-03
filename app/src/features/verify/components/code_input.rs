use leptos::prelude::*;

#[component]
pub fn CodeInput(value: RwSignal<String>) -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let char_at = move |i: usize| value.get().chars().nth(i);

    let focus_input = move |_| {
        if let Some(el) = input_ref.get() {
            let _ = el.focus();
        }
    };

    view! {
        <div class="flex flex-col gap-4">

            // -- slots visuales + input overlay en posicion relativa
            <div class="relative flex items-center justify-center">

                // -- slots visuales (pointer-events-none para que el click pase al input)
                <div
                    class="flex items-center justify-center gap-3 cursor-text pointer-events-none"
                >
                    <div class="flex gap-2">
                        {move || (0..3usize).map(|i| {
                            let ch = char_at(i);
                            view! { <Slot ch=ch /> }
                        }).collect_view()}
                    </div>
                    <span class="text-gray-400 font-bold text-xl select-none pb-1">"-"</span>
                    <div class="flex gap-2">
                        {move || (3..6usize).map(|i| {
                            let ch = char_at(i);
                            view! { <Slot ch=ch /> }
                        }).collect_view()}
                    </div>
                </div>

                // -- input real encima, completamente transparente
                <input
                    node_ref=input_ref
                    type="text"
                    maxlength="6"
                    autocomplete="off"
                    class="absolute inset-0 w-full h-full opacity-0 cursor-text"
                    on:input=move |ev| {
                        let cleaned: String = event_target_value(&ev)
                            .chars()
                            .filter(|c| c.is_alphanumeric())
                            .map(|c| c.to_ascii_uppercase())
                            .take(6)
                            .collect();
                        value.set(cleaned);
                    }
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        let key = ev.key();
                        if matches!(key.as_str(), "Backspace" | "Tab" | "ArrowLeft" | "ArrowRight" | "Delete") {
                            return;
                        }
                        if key.len() == 1 {
                            if let Some(c) = key.chars().next() {
                                if !c.is_alphanumeric() {
                                    ev.prevent_default();
                                }
                            }
                        }
                    }
                    prop:value=move || value.get()
                />
            </div>

            // -- hint clickeable para enfocar
            <p
                class="text-xs text-gray-400 text-center cursor-text"
                on:click=focus_input
            >
                "Click and start typing"
            </p>
        </div>
    }
}

#[component]
fn Slot(ch: Option<char>) -> impl IntoView {
    let filled = ch.is_some();
    let display = ch.map(|c| c.to_string()).unwrap_or_else(|| "_".to_string());

    view! {
        <div class=format!(
            "w-11 h-14 flex items-end justify-center pb-2 font-mono font-bold text-xl transition-all duration-150 border-b-2 {}",
            if filled { "border-primary-500 text-primary-600" } else { "border-gray-300 text-gray-300" }
        )>
            {display}
        </div>
    }
}

/// Formatea 6 chars sin guion → "ABC-DEF"
pub fn format_code(raw: &str) -> String {
    let clean: String = raw
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .take(6)
        .collect();

    if clean.len() == 6 {
        format!("{}-{}", &clean[..3], &clean[3..])
    } else {
        clean
    }
}
