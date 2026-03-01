use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::features::{
    documents::page::DocumentsPage, sign::page::SignPage, verify::page::VerifyPage,
};
use crate::shared::components::navbar::Navbar;

#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
                <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
                // -- css compilado por cargo-leptos desde style/main.css
                <link rel="stylesheet" href="/pkg/stego-app.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="StegoSign — Document Integrity"/>
        <Router>
            <Navbar/>
            <main class="min-h-screen pt-16 px-4 max-w-6xl mx-auto">
                <Routes fallback=|| view! {
                    <p class="text-red-400 text-center mt-20 text-xl">"Page not found"</p>
                }>
                    <Route path=path!("/")          view=SignPage/>
                    <Route path=path!("/verify")    view=VerifyPage/>
                    <Route path=path!("/documents") view=DocumentsPage/>
                </Routes>
            </main>
        </Router>
    }
}
