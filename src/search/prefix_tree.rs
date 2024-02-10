use std::collections::HashMap;
use crate::search::types::KBarAction;

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
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }

        current.is_end_of_word = true;
    }

    pub fn batch_insert(actions: &[KBarAction]) -> Self {
        let mut trie = Trie::new();
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

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
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

        self.collect_words(current, prefix.to_string(), &mut result);
        result
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
