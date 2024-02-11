use crate::search::kbar_search::{Content, SearchBar};
use leptos::*;
use leptos_hotkeys::prelude::*;
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};

pub const CONTENT_HEADER_CSS: &'static str =
    "uppercase text-xs tracking-tight text-gray-400 font-semibold px-4 py-1 mt-1";
pub const CONTENT_ITEM_CSS: &'static str = "hover:bg-gray-100 dark:hover:bg-[#222222] cursor-pointer flex justify-between items-center w-full p-4 transition ease-in duration-100";

#[component]
pub fn KBarModal(
    show_kbar: RwSignal<bool>,
    show_theme: bool,
    themekey: &'static str,
) -> impl IntoView {
    let (search_input, set_search_input) = create_signal("".to_string());

    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().initial_value(ColorMode::Dark));

    if show_theme {
        use_hotkeys!((themekey, "kbar") => move |_| {
            if mode.get() == ColorMode::Light {
                set_mode.set(ColorMode::Dark);
            } else {
                set_mode.set(ColorMode::Light);
            }
        });
    }

    view! {
        <div
            on:click=move |ev| {
                ev.stop_propagation();
            }

            class="bg-white dark:bg-[#1a1a1a] dark:text-white w-[550px] shadow-lg z-99 rounded-md"
        >
            <SearchBar search_input=search_input set_search_input=set_search_input/>
            <Content search_input=search_input/>
            <Show when=move || { show_theme }>
                <p class=CONTENT_HEADER_CSS>Preferences</p>
                <div
                    class=CONTENT_ITEM_CSS
                    on:click=move |_| {
                        if mode.get() == ColorMode::Light {
                            set_mode.set(ColorMode::Dark);
                        } else {
                            set_mode.set(ColorMode::Light);
                        }
                    }
                >

                    <p>Change theme...</p>

                    <div class="flex space-x-2 items-center">

                        {
                            let scuts = themekey
                                .split("+")
                                .into_iter()
                                .map(|s| {
                                    view! { <code class="">{s}</code> }
                                })
                                .collect::<Vec<_>>();
                            scuts
                        }

                    </div>
                </div>
            </Show>
        </div>
    }
}
