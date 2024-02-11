use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{
    use_color_mode_with_options,ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};
use leptos::html::html;
use leptos_kbar::prelude::*;
use crate::info_page::InfoPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();


    let kbar_actions = vec![
        KBarAction::new(1, "Source Code".to_string(), "g+h".to_string(), vec!["github".to_string(), "repository".to_string(), "source code".to_string()], Callback::new( move |_| {
            window().location().set_href("https://github.com/friendlymatthew/leptos-kbar").expect("Failed to navigate");
        })),
        KBarAction::new(2, "Learn more".to_string(), "l".to_string(), vec!["contribute".to_string(), "documentation".to_string()], Callback::new(move |_| {
            window().location().set_href("/information").expect("failed to go to /information");
        })),
    ];

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>

        <KBarProvider actions=kbar_actions show_theme=true>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/information" view=InfoPage/>
                    <Route path="/:else" view=ErrorPage/>
                </Routes>
            </Router>
        </KBarProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .initial_value(ColorMode::from(html().class_name())),
    );

    const REPO: &'static str = "https://github.com/friendlymatthew/leptos-kbar";

    view! {
        <main class="bg-white dark:bg-[#1a1a1a] dark:text-white h-screen py-20 flex justify-center font-robotomono">
            <div class="w-10/12">
                <p class="text-xl">leptos kbar</p>
                <p>"Press `ctrl+k`"</p>
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
