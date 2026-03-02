use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::features::{
    documents::page::DocumentsPage, home::page::HomePage, sign::page::SignPage,
    verify::page::VerifyPage,
};
use crate::shared::components::{footer::Footer, navbar::Navbar};

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
                <style type="text/tailwindcss">
                    "@theme {
                        --font-body:              'Inter', sans-serif;
                        --font-display:           'Poppins', sans-serif;
                        --color-primary-50:       #fef2f2;
                        --color-primary-400:      #e83d61;
                        --color-primary-500:      #d20f39;
                        --color-primary-600:      #b00d30;
                        --color-primary-700:      #8f0a27;
                        --color-accent:           #f59e0b;
                        --color-accent-light:     #fbbf24;
                        --color-accent-dark:      #d97706;
                        --color-navy:             #1e293b;
                        --color-background-light: #ffffff;
                    }
                    html { scroll-behavior: smooth; }
                    body {
                        font-family: var(--font-body);
                        color: #374151;
                        background-color: #ffffff;
                        -webkit-font-smoothing: antialiased;
                    }
                    h1, h2, h3, h4, h5, h6 {
                        font-family: var(--font-display);
                        font-weight: 700;
                    }"
                </style>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700;800&family=Inter:wght@300;400;500;600&display=swap" rel="stylesheet"/>
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
            <main class="min-h-screen pt-16">
                <Routes fallback=|| view! {
                    <div class="flex flex-col items-center justify-center min-h-screen gap-4">
                        <span class="text-6xl">"🔍"</span>
                        <h1 class="text-2xl font-display font-bold text-navy">"Page not found"</h1>
                        <a href="/" class="text-primary-500 hover:text-primary-600 font-medium">
                            "← Back to home"
                        </a>
                    </div>
                }>
                    <Route path=path!("/")          view=HomePage/>
                    <Route path=path!("/sign")      view=SignPage/>
                    <Route path=path!("/verify")    view=VerifyPage/>
                    <Route path=path!("/documents") view=DocumentsPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
