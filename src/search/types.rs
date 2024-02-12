use leptos::Callback;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref NEXT_ID: Mutex<usize> = Mutex::new(1);
}

#[derive(Debug, Clone)]
pub struct KBarAction {
    pub(crate) id: Arc<usize>,        // we'll need this to be unique
    pub(crate) name: Arc<String>,     // we'll require this to also be unique
    pub(crate) shortcut: Arc<String>, // we'll need to require this to not contain commas (only  key or "+" symbols)
    pub(crate) keywords: Vec<Arc<String>>,
    pub(crate) perform: Callback<()>,
}

impl KBarAction {
    pub fn new(
        name: String,
        shortcut: String,
        keywords: Vec<String>,
        perform: Callback<()>,
    ) -> Arc<Self> {
        let mut id_guard = NEXT_ID.lock().unwrap();
        let id = *id_guard;
        *id_guard += 1;

        let keywords = keywords.iter().map(|k| Arc::new(k.clone())).collect();
        Arc::new(KBarAction {
            id: Arc::new(id),
            name: Arc::new(name),
            shortcut: Arc::new(shortcut),
            keywords,
            perform,
        })
    }

    pub fn flatten(action_ref: &Arc<KBarAction>) -> Vec<(Arc<String>, Arc<KBarAction>)> {
        std::iter::once((action_ref.name.clone(), action_ref.clone()))
            .chain(
                (action_ref)
                    .keywords
                    .iter()
                    .map(|k| (k.clone(), action_ref.clone())),
            )
            .collect()
    }
}

impl Hash for KBarAction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.id).hash(state);
        (*self.name).hash(state);
    }
}
impl Eq for KBarAction {}
impl PartialEq for KBarAction {
    fn eq(&self, other: &Self) -> bool {
        *self.id == *other.id
    }
}
