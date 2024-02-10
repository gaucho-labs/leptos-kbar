use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::ThemeProvider;

use leptos_kbar::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();


    let kbar_actions = vec![
        KBarAction::new("contribute".to_string(), "Contribute".to_string(), "c".to_string(), vec!["github".to_string(), "repository".to_string(), "source code".to_string()], Callback::new( move |_| {
            window().location().set_href("https://github.com/friendlymatthew/leptos-kbar").expect("Failed to navigate");
        })),
        KBarAction::new("howdy".to_string(), "howdy".to_string(), "h".to_string(), vec!["cowboy".to_string(), "deez".to_string()], Callback::new(move |_| {
            logging::log!("called howdy");
        })),
        KBarAction::new("deeznuts".to_string(), "deeznuts".to_string(), "d".to_string(), vec!["cholo".to_string(), "que".to_string()], Callback::new(move |_| {
            logging::log!("called deeznuts");
        })),
    ];

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>
        <KBarProvider
            actions=kbar_actions
        >
            <ThemeProvider>
                <Router>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/:else" view=ErrorPage/>
                    </Routes>
                </Router>
            </ThemeProvider>
        </KBarProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    const REPO: &'static str = "https://github.com/friendlymatthew/leptos-kbar";

    view! {
        <main class="dark:bg-[#1a1a1a] bg-white dark:text-white h-screen py-20 w-full space-y-8 font-robotomono">
            <div class="text-center space-y-2">
                <p class="text-3xl">leptos-csr-starter-kit</p>
                <p>Set up a client side rendered Leptos app with one command</p>
            </div>
        </main>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class=format!(
            "h-screen w-full flex flex-col items-center justify-center font-robotomono",
        )>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
