use leptos::*;
use leptos_hotkeys::prelude::*;

use crate::kbar_modal::KBarModal;
use crate::search::prefix_tree::Trie;
use crate::search::types::Action;

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

#[derive(Clone)]
pub struct KBarContext {
    pub actions: RwSignal<Vec<Action>>,
    pub tree: RwSignal<Trie>,
}

pub fn use_kbar_context() -> KBarContext {
    use_context::<KBarContext>().expect("expected kbar context")
}


#[component]
pub fn KBarProvider(
    #[prop(default = "control+k")] hotkey: &'static str,

    #[prop(default = "escape")] escape: &'static str,

    children: Children,
) -> impl IntoView {
    logging::log!("debug:inside kbar provider");

    let actions = vec![
        Action::new("chair".to_string(), "chair".to_string(), "c+h".to_string(), vec!["cheese".to_string(), "dinosaur".to_string()], Callback::new( move |_| {
            logging::log!("called chair");

        })),
        Action::new("howdy".to_string(), "howdy".to_string(), "h".to_string(), vec!["cowboy".to_string(), "deez".to_string()], Callback::new(move |_| {
            logging::log!("called howdy");
        })),
        Action::new("deeznuts".to_string(), "deeznuts".to_string(), "d".to_string(), vec!["cholo".to_string(), "que".to_string()], Callback::new(move |_| {
            logging::log!("called deeznuts");
        })),
    ];

    let tree = Trie::batch_insert(&actions);

    let actions = create_rw_signal(actions);
    let tree = create_rw_signal(tree);

    provide_context(KBarContext {
        actions,
        tree,
    });


    view! {
        <HotkeysProvider>
            <KBarPositioner hotkey=hotkey escape=escape/>
            {children()}
        </HotkeysProvider>
    }
}
