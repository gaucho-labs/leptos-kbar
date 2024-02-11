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
pub fn Action(
    curr_index: RwSignal<usize>,
    index: usize,
    action: Arc<KBarAction>
) -> impl IntoView {
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

            class=move || {
                if curr_index.get() == index + 1 {
                    format!("{} bg-gray-100 dark:bg-[#222222]", CONTENT_ITEM_CSS)
                } else {
                    format!("{} ", CONTENT_ITEM_CSS)
                }
            }
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
                <Show when=move || { curr_index.get() == index + 1 }>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 16 16"
                        fill="currentColor"
                        class="w-4 h-4"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M6.22 4.22a.75.75 0 0 1 1.06 0l3.25 3.25a.75.75 0 0 1 0 1.06l-3.25 3.25a.75.75 0 0 1-1.06-1.06L8.94 8 6.22 5.28a.75.75 0 0 1 0-1.06Z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </Show>
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

    // since you can navigate with arrow keys, we're going to create a state
    // but let's also make it where the content idx + 1 == curr_index since 0 is the search bar
    let curr_index = create_rw_signal(0);

    use_hotkeys!(("enter", "kbar") => move |_| {
        let curr_index = curr_index.get();

        if curr_index != 0 {
            // then we know it's at a valid state

            // we should also check if it's a parent
                // then we can rerender result to display the children's prefix tree

            // else
            if let Some(action_callback) = result.get().get(curr_index - 1) {
                let (_idx, a) = action_callback;
                Callable::call(&a.perform, ());
            }

        }
    });

    use_hotkeys!(("arrowup", "kbar") => move |_| {
        curr_index.update(move |i| {
            if *i == 1 {
                *i = result.get().len();
            } else {
                *i -= 1;
            }
        });
    });

    use_hotkeys!(("arrowdown", "kbar") => move |_| {
        curr_index.update(move |i| {
            if *i == result.get().len() {
                *i = 1;
            } else {
                *i += 1;
            }
        });
    });

    view! {
        <ul>
            <p class=CONTENT_HEADER_CSS>Navigation</p>
            <For
                each=result
                key=|(idx, action)| action.clone()
                children=move |(idx, action_ref)| {
                    view! { <Action curr_index=curr_index index=idx action=action_ref.clone()/> }
                }
            />

        </ul>
    }
}
