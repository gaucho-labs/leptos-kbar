use crate::kbar_modal::KBarModal;
use crate::search::prefix_tree::Trie;
use crate::search::types::KBarAction;
use leptos::*;
use leptos_hotkeys::prelude::*;
use std::sync::Arc;

#[component]
pub fn KBarPositioner(
    hotkey: &'static str,
    escapekey: &'static str,
    show_theme: bool,
    themekey: &'static str,
) -> impl IntoView {
    let show_kbar = create_rw_signal(false);
    let HotkeysContext {
        enable_scope,
        disable_scope,
        ..
    } = use_hotkeys_context();
    let KBarContext { actions, .. } = use_kbar_context();

    for action in &actions.get() {
        use_hotkeys!((&action.shortcut.clone(), "kbar") => action.perform);
    }

    use_hotkeys!((hotkey) => move |_| {
        if show_kbar.get() {
            show_kbar.set(false);
            disable_scope("kbar".to_string());
        } else {
            show_kbar.set(true);
            enable_scope("kbar".to_string());
        }
    });

    use_hotkeys!((escapekey) => move |_| {
        show_kbar.set(false);
    });

    view! {
        <div class="leptos-kbar">
            <Show when=move || show_kbar.get()>
                <div
                    on:click=move |_| { show_kbar.set(false) }
                    class="fixed inset-0 flex items-center justify-center z-50 px-8 bg-smoke"
                >
                    <KBarModal show_kbar=show_kbar show_theme=show_theme themekey=themekey/>
                </div>
            </Show>
        </div>
    }
}

#[derive(Clone)]
pub struct KBarContext {
    pub actions: RwSignal<Vec<Arc<KBarAction>>>,
    pub tree: RwSignal<Trie>,
}

pub fn use_kbar_context() -> KBarContext {
    use_context::<KBarContext>().expect("expected kbar context")
}

#[component]
pub fn KBarProvider(
    #[prop(default = "control+k")] hotkey: &'static str,
    #[prop(default = "escape")] escapekey: &'static str,

    #[prop(default=true)] show_theme: bool,
    #[prop(default = "t")] themekey: &'static str,
    actions: Vec<Arc<KBarAction>>,
    children: Children,
) -> impl IntoView {

    let tree = Trie::batch_insert(&actions);

    let actionsList = create_rw_signal(actions);
    let tree = create_rw_signal(tree);

    provide_context(KBarContext {
        actions: actionsList,
        tree,
    });

    view! {
        <style>{include_str!("./styles.css")}</style>

        <HotkeysProvider>
            <KBarPositioner
                hotkey=hotkey
                escapekey=escapekey
                show_theme=show_theme
                themekey=themekey
            />
            {children()}
        </HotkeysProvider>
    }
}
