use crate::kbar_modal::{CONTENT_HEADER_CSS, CONTENT_ITEM_CSS};
use crate::kbar_provider::{use_kbar_context, KBarContext};
use crate::prelude::KBarAction;
use leptos::*;
use leptos_hotkeys::hotkeys_provider::HotkeysContext;
use leptos_hotkeys::prelude::*;
use std::sync::Arc;

#[component]
pub fn SearchBar(
    search_input: ReadSignal<String>, // @justbobinaround -- this is an optimization we can do
    set_search_input: WriteSignal<String>,
) -> impl IntoView {
    let HotkeysContext {
        enable_scope,
        disable_scope,
        ..
    } = use_hotkeys_context();

    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_search_input(event_target_value(&ev));
            }

            on:focus=move |_| {
                disable_scope("kbar".to_string());
            }

            on:blur=move |_| {
                enable_scope("kbar".to_string());
            }

            placeholder="Type a command or search..."
            prop:value=search_input
            class="p-4 w-full focus:outline-none dark:bg-[#1a1a1a] dark:text-white"
        />
    }
}

#[component]
pub fn Action(action: Arc<KBarAction>) -> impl IntoView {
    let shortcuts = action
        .shortcut
        .clone()
        .split("+")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let name = action.name.clone().to_string();

    view! {
        <div
            on:click=move |_| {
                logging::log!("action clicked");
                Callable::call(&action.perform, ());
                ()
            }

            class=CONTENT_ITEM_CSS
        >
            <p>{name}</p>
            <div class="flex space-x-2 items-center">

                {
                    let scuts = shortcuts
                        .iter()
                        .map(|s| {
                            view! { <code class="">{s}</code> }
                        })
                        .collect::<Vec<_>>();
                    scuts
                }

            </div>
        </div>
    }
}

#[component]
pub fn Content(search_input: ReadSignal<String>) -> impl IntoView {
    let KBarContext { tree, .. } = use_kbar_context();

    let result = create_rw_signal(vec![]);

    create_effect(move |_| {
        let trie_results = tree.get().starts_with(&search_input.get());
        logging::log!("results: {:?}", trie_results);
        result.set(trie_results);
    });

    let curr_index = create_rw_signal(0);

    use_hotkeys!(("arrowup", "kbar") => move |_| {
        curr_index.update(move |i| {
            *i += 1;
        });
    });

    use_hotkeys!(("arrowdown", "kbar") => move |_| {
        curr_index.update(move |i| {
            *i -= 1;
        });
    });

    view! {
        <ul>
            <p class=CONTENT_HEADER_CSS>Navigation</p>
            <For
                each=result
                key=|action| action.clone()
                children=move |action_ref| {
                    view! { <Action action=action_ref.clone()/> }
                }
            />

        </ul>
    }
}
