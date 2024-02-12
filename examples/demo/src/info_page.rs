use leptos::*;
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn
};
use leptos::html::html;



#[component]
pub fn InfoPage(

) -> impl IntoView {

    let UseColorModeReturn { .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .initial_value(ColorMode::from(html().class_name())),
    );

    let content: Vec<(&'static str, &'static str)> = vec![
        ("Support nested action routing", "For a given action, there may exist further routing. This is where subactions come into play. When we click into an action, we can rerender the list to display that action's subactions."),
        ("Optional headless component", "Currently I wrote the styling for this kBar. But it would be nicer and more extensible to provide an unstyled command menu."),
        ("Integrations with other libraries", "The theme switcher uses leptos-use's `use-color-mode` as that seems to be the most robust theme switcher in the Leptos ecosystem. We also want to empower users with easily configurable extensions to great libraries like leptos-use."),
        ("Actually use cmd+K", "Yes, this has already been solved and will be implemented when we release the next version of leptos-hotkeys")
    ];

    view! {
        <div class="dark:bg-[#1a1a1a] dark:text-white flex w-full justify-center">
            <div class="w-10/12 mt-12 font-robotomono">
                <div class="">
                    <p class="text-2xl font-semibold">Roadmap</p>
                    <p>leptos kbar is a fully extensible interface for your leptos applications.</p>
                    <a href="https://github.com/friendlymatthew/leptos-kbar" class="underline">Come build!</a>
                    <br/><br/>
                    <p>Here are a couple of things that I had in mind:</p>
                    {
                        let cmap = content.into_iter().map(|(title, desc)| {
                        view! {
                            <div class="py-2 w-8/12">
                                <strong class="text-lg">- {title}</strong>
                                <p class="text-base">{desc}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>();

                        cmap
                    }
                </div>
            <p class="my-20">"Although I plan on working on this, it's always fun to work with other people, so if this excites you, feel free to message me on discord or solve an issue on Github!"</p>

            </div>
        </div>
    }
}