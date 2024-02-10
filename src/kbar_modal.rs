use crate::search::kbar_search::KBarSearch;
use leptos::*;

#[component]
pub fn KBarModal(show_kbar: RwSignal<bool>) -> impl IntoView {

    view! {
        <div
            on:click=move |ev| {
                ev.stop_propagation();
            }

            class="modal"
        >
            <KBarSearch/>
        </div>
    }
}
