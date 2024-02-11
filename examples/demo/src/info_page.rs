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


    view! {
        <div class="dark:bg-[#1a1a1a] dark:text-white flex w-full justify-center">
            <div class="w-10/12 mt-12">
                <div class="font-robotomono">
                    <p class="text-xl">leptos_kbar</p>
                    <p>leptos kbar is a fully extensible interface for your leptos applications.</p>
                </div>
            </div>
        </div>
    }
}