// Naive implementation of a trie

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct NaiveTrieNode {
    is_terminal: bool,
    is_root: bool,
    value: Option<char>,
    children: HashMap<char, NaiveTrieNode>,
}

struct NaiveTrie {
    root: NaiveTrieNode,
}

impl NaiveTrie {
    fn new() -> Self {
        Self {
            root: NaiveTrieNode {
                is_terminal: false,
                is_root: true,
                value: None,
                children: HashMap::new(),
            },
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(NaiveTrieNode {
                is_terminal: false,
                value: Some(c),
                children: HashMap::new(),
                is_root: false,
            });
        }
        current.is_terminal = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            let next = current.children.get(&c);
            match next {
                Some(next_node) => current = next_node,
                None => return false,
            }
        }
        current.is_terminal
    }

    fn delete(&mut self, word: &str) {
        recursively_delete_node(&mut self.root, word);
    }
}

fn recursively_delete_node(node: &mut NaiveTrieNode, word: &str) -> Option<NaiveTrieNode> {
    if word.is_empty() && node.is_terminal {
        node.is_terminal = false;
        if !node.children.is_empty() {
            return Some(node.clone());
        }
        node.value = None;
        return None;
    }
    // Recursively delete the node
    let next_char = word.chars().next().unwrap();

    let next_node = node.children.get_mut(&next_char).unwrap();
    let new_node = recursively_delete_node(next_node, &word[1..]);
    match new_node {
        None => {
            node.children.remove(&next_char);
            if node.children.is_empty() {
                node.value = None;
                return None;
            }
        }
        Some(trie_node) => {
            node.children.insert(next_char, trie_node);
        }
    };

    Some(node.clone())
}

fn visualize_trie(node: &NaiveTrieNode, prefix: &str, is_last: bool) {
    // Print the current node
    let marker = if is_last { "└── " } else { "├── " };
    let value = node.value.map_or("ROOT".to_string(), |c| c.to_string());
    let terminal = if node.is_terminal { " (T)" } else { "" };
    println!("{}{}{}{}", prefix, marker, value, terminal);

    // Calculate the new prefix for children
    let new_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    // Sort children for consistent visualization
    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by_key(|(c, _)| *c);

    // Print children
    for (i, (_, child)) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        visualize_trie(child, &new_prefix, is_last_child);
    }
}

fn main() {
    let mut trie = NaiveTrie::new();
    trie.insert("hello");
    trie.insert("hell");
    trie.insert("world");
    trie.insert("hi");
    trie.insert("wow");

    println!("Trie Structure:");
    visualize_trie(&trie.root, "", true);

    println!("{:?}", trie.search("hello"));
    println!("{:?}", trie.search("hell"));
    println!("{:?}", trie.search("world"));
    println!("{:?}", trie.search("hi"));
    trie.delete("hell");
    println!("{:?}", trie.search("hello"));
    println!("{:?}", trie.search("hell"));

    println!("Trie Structure after deletion:");
    visualize_trie(&trie.root, "", true);

    trie.delete("hello");
    println!("Trie Structure after deletion:");
    visualize_trie(&trie.root, "", true);
}
