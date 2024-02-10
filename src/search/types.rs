use std::collections::BTreeMap;
use leptos::Callback;

#[derive(Debug, Clone)]
pub struct Action {
    id: String,   // we'll need this to be unique
    pub(crate) name: String, // we'll require this to also be unique
    shortcut: String,
    pub(crate) keywords: Vec<String>,
    perform: Callback<()>,
}

impl Action {

    pub(crate) fn new(id: String, name: String, shortcut: String, keywords: Vec<String>, perform: Callback<()>) -> Self {
        Action {
            id,
            name,
            shortcut,
            keywords,
            perform
        }
    }

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
    pub fn to_searchable(&self) -> Vec<(String, String)> {
        self.content
            .iter()
            .flat_map(|a| a.flatten())
            .collect::<Vec<(String, String)>>()
    }
}
