use std::sync::Arc;
use leptos::*;
use leptos_hotkeys::hotkeys_provider::HotkeysContext;
use leptos_hotkeys::prelude::use_hotkeys_context;
use crate::kbar_provider::{ KBarContext, use_kbar_context };
use crate::prelude::KBarAction;

#[component]
pub fn KBarSearch() -> impl IntoView {
    let (search_input, set_search_input) = create_signal("".to_string());

    view! {
        <SearchBar search_input=search_input set_search_input=set_search_input/>
        <Content search_input=search_input/>
    }
}

#[component]
pub fn SearchBar(
    search_input: ReadSignal<String>, // @justbobinaround -- this is an optimization we can do
    set_search_input: WriteSignal<String>
) -> impl IntoView {

    let HotkeysContext { enable_scope, disable_scope, .. } = use_hotkeys_context();

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
            class="searchbar"
        />
    }
}



#[component]
pub fn Action(action: Arc<KBarAction>) -> impl IntoView {
    let shortcuts = action.shortcut.clone().split("+").map(|s| s.to_string()).collect::<Vec<String>>();

    view! {
        <div class="action-content">
            <p>{&*(action.name)}</p>
            <div class="action-shortcut-content">

                {
                    let scuts = shortcuts
                        .iter()
                        .map(|s| {
                            view! { <p>{s}</p> }
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

    view! {
        <ul>
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
