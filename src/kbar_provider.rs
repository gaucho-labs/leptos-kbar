use leptos::*;
use leptos_hotkeys::prelude::*;

use crate::kbar_modal::KBarModal;

#[component]
pub fn KBarPositioner(hotkey: &'static str, escape: &'static str) -> impl IntoView {
    let show_kbar = create_rw_signal(false);

    use_hotkeys!((hotkey) => move |_| {
        logging::log!("howdy");

        if show_kbar.get() {
            show_kbar.set(false);
        } else {
            show_kbar.set(true);
        }
    });

    use_hotkeys!((escape) => move |_| {
        show_kbar.set(false);
    });

    view! {
        <Show when=move || show_kbar.get()>
            <div
                on:click=move |_| { show_kbar.set(false) }
                style="\
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background-color: rgba(0, 0, 0, 0.1);
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 999;"
            >
                <KBarModal show_kbar=show_kbar/>
            </div>
        </Show>
    }
}

#[component]
pub fn KBarProvider(
    #[prop(default = "control+k")] hotkey: &'static str,

    #[prop(default = "escape")] escape: &'static str,

    children: Children,
) -> impl IntoView {
    logging::log!("debug:inside kbar provider");
    view! {
        <HotkeysProvider>
            <KBarPositioner hotkey=hotkey escape=escape/>
            {children()}
        </HotkeysProvider>
    }
}
