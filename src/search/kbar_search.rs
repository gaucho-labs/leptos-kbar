use leptos::*;
use std::collections::BTreeMap;



#[component]
pub fn KBarSearch() -> impl IntoView {
    let (search_input, set_search_input) = create_signal("".to_string());

    view! {
        <SearchBar
            search_input=search_input
            set_search_input=set_search_input
        />
        <Content/>
    }
}

#[component]
pub fn SearchBar(
    search_input: ReadSignal<String>, // @justbobinaround -- this is an optimization we can do
    set_search_input: WriteSignal<String>,
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
pub fn Content() -> impl IntoView {
    view! {
        <div>



        </div>
    }
}
