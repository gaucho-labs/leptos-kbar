use std::collections::{HashMap};
use leptos::Callback;



#[derive(Debug, Clone)]
pub struct KBarAction {
    pub(crate) id: usize, // we'll need this to be unique
    pub(crate) name: String, // we'll require this to also be unique
    pub(crate) shortcut: String,
    pub(crate) keywords: Vec<String>,
    pub(crate) perform: Callback<()>,
}

impl KBarAction {
    pub fn new(
        id: usize,
        name: String,
        shortcut: String,
        keywords: Vec<String>,
        perform: Callback<()>
    ) -> Self {
        KBarAction {
            id,
            name,
            shortcut,
            keywords,
            perform,
        }
    }

    fn flatten(&self) -> Vec<(String, usize)> {
        std::iter
            ::once((self.name.clone(), self.id.clone()))
            .chain(self.keywords.iter().map(|k| (k.clone(), self.id.clone())))
            .collect()
    }
}
