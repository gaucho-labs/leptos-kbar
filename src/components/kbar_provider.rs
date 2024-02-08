use leptos::*;
use leptos_hotkeys::prelude::*;


pub struct Action {
    id: String,
    name: String,
    shortcut: Vec<String>,
    keywords: Vec<String>,
    perform: Callback<()>
}


#[component]
pub fn KBarProvider(

    actions: Action,

    // maybe we need to add a list of scopes where we don't want the kbar to appear
    // disable: Vec<String>,

    children: Children
) -> impl IntoView {

    use_hotkeys!(("meta+K") => move |_| {
        logging::log!("trigger k bar!");
    });

    view!{
        <div>
            {children()}
        </div>
    }
}