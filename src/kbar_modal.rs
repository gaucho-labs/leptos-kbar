use crate::search::kbar_search::KBarSearch;
use leptos::*;
use leptos_hotkeys::prelude::*;

#[component]
pub fn KBarModal(show_kbar: RwSignal<bool>) -> impl IntoView {
    use_hotkeys!(("", "kbar") => move |_| {

    });

    view! {
        <div
            on:click=move |ev| {
                ev.stop_propagation();
            }

            style="\
            background-color: #fff;
            width: 500px;
            padding: 6px;
            border-radius: 5px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            z-index: 1000;
            "
        >
            <KBarSearch />
        </div>
    }
}
