use leptos::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Action {
    id: String,   // we'll need this to be unique
    name: String, // we'll require this to also be unique
    shortcut: String,
    keywords: Vec<String>,
    perform: Callback<()>,
}

impl Action {
    fn flatten(&self) -> Vec<(String, String)> {
        std::iter::once((self.name.clone(), self.id.clone()))
            .chain(self.keywords.iter().map(|k| (k.clone(), self.id.clone())))
            .collect()
    }
}

pub struct Search {
    content: Vec<Action>,
}

impl Search {
    /// to_searchable() takes in a list of actions and transform it into a searchable btree structure
    /// what can we search?
    /// right now:
    /// - name
    /// - keyword
    pub fn to_searchable(&self) -> BTreeMap<String, String> {
        self.content
            .iter()
            .flat_map(|a| a.flatten())
            .collect::<BTreeMap<String, String>>()
    }
}

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
