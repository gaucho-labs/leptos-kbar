use leptos::*;
use crate::kbar_provider::{ KBarContext, use_kbar_context };
use crate::search::types::KBarAction;

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
    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_search_input(event_target_value(&ev));
            }

            placeholder="Type a command or search..."
            prop:value=search_input
            style="\
            font-size: 1rem;
            padding: 0.25rem;
            width: 100%;
            outline: none;
            "
        />
    }
}

#[component]
pub fn Content(search_input: ReadSignal<String>) -> impl IntoView {
    let KBarContext { tree, actions_table, .. } = use_kbar_context();

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
                key=|idx| idx.clone()
                children=move |idx| {
                    let binding = actions_table.get();
                    let action_opt = binding.get(&idx);
                    match action_opt {
                        Some(action) => {
                            view! { <div>{&action.name}</div> }
                        }
                        None => {
                            view! { <div></div> }
                        }
                    }
                }
            />

        </ul>
    }
}
