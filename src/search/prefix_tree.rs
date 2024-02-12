use crate::search::types::KBarAction;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
    indexing: HashMap<Arc<String>, Arc<KBarAction>>,
}

impl Trie {
    pub(crate) fn new(actions: &Vec<Arc<KBarAction>>) -> Self {
        // todo! figure out an optimal way. this is fucking disgusting.
        // so many clones. we'll add a lifetime of <'str>
        // we flatten out the names and keywords for every action and map it to the id

        let mut indexer = HashMap::new();

        for action in actions {
            let action_refs = KBarAction::flatten(action);

            for (keyword_ref, action_ref) in action_refs {
                indexer.insert(keyword_ref, action_ref);
            }
        }

        Trie {
            root: TrieNode::new(),
            indexing: indexer,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }

        current.is_end_of_word = true;
    }

    pub fn batch_insert(actions: &Vec<Arc<KBarAction>>) -> Self {
        let mut trie = Trie::new(actions);
        for action in actions {
            // Insert the action name with its own name as the identifier
            trie.insert(&action.name);
            // Insert each keyword with the action's name as the identifier
            for keyword in &action.keywords {
                trie.insert(keyword);
            }
        }
        trie
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<(usize, Arc<KBarAction>)> {
        let mut result = Vec::new();
        let mut current = &self.root;

        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => {
                    current = node;
                }
                None => {
                    return Vec::new();
                }
            }
        }

        let mut word_ids = HashSet::new();
        self.collect_words(current, prefix.to_string(), &mut result);

        for word in result {
            if let Some(action) = self.indexing.get(&word) {
                word_ids.insert(action.clone());
            }
        }

        let mut actions: Vec<_> = word_ids.into_iter().collect();
        actions.sort_by_key(| a| *a.id);


        actions.into_iter().enumerate().collect()
    }

    fn collect_words(&self, node: &TrieNode, prefix: String, result: &mut Vec<String>) {
        if node.is_end_of_word {
            result.push(prefix.clone());
        }

        node.children.iter().for_each(|(ch, child)| {
            let new_prefix = format!("{}{}", prefix, ch);
            self.collect_words(child, new_prefix, result);
        });
    }
}
