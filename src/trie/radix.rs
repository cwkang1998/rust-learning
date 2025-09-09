// Implementing a radix trie

// Radix tree is similar to a normal trie but each node can have more than one character.
// Its also known as a compressed trie.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct RadixTrieNode {
    children: HashMap<String, RadixTrieNode>,
    is_terminal: bool,
}

#[derive(Debug, Default)]
pub struct RadixTrie {
    root: RadixTrieNode,
}

impl RadixTrie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, input_word: &str) {
        if input_word.is_empty() {
            return;
        }

        let mut current: &mut RadixTrieNode = &mut self.root;
        let mut current_word = input_word;

        while !current_word.is_empty() {
            // Find common prefix and potential next node.
            let next_keys = current.children.keys().find_map(|child_key| {
                let common_prefix = get_common_prefix(current_word, child_key);
                if !common_prefix.is_empty() {
                    return Some((child_key.to_owned(), common_prefix.to_owned()));
                }
                None
            });

            // With the common prefix and the next node known, we now go through the cases.]
            // 1. if no common prefix, insert the node as it is /
            // 2. if common prefix == input word length, return immediately (node already exists)
            // 3. if common prefix length == value length of the nodes, continue down (continue searching)
            // 4. if common prefix not equals, then split the node into its common prefix and its postfix(reorg).
            let (next_possible_node, next_key) = match next_keys {
                None => {
                    // Case 1: no common prefix, insert new word whole from root.
                    current.children.insert(
                        current_word.to_string(),
                        RadixTrieNode {
                            is_terminal: true,
                            children: HashMap::new(),
                        },
                    );
                    break;
                }
                Some((next_possible_node, next_key)) => (next_possible_node, next_key),
            };

            if next_possible_node == current_word {
                // Case 2: common prefix exists as a node, and its the same as the input, thus the node already exists, and we don't insert.
                current_word = ""
            } else if next_key == next_possible_node {
                // Case 3: there's already a node of the prefix, so we continue searching
                let next_node = current.children.get_mut(&next_possible_node).unwrap();
                current_word = &current_word[next_key.len()..];
                current = next_node;
            } else {
                // Case 4: There's a common prefix, and an existing node, we split the node and reorg the tree.
                let current_next_node = current.children.remove(&next_possible_node).unwrap();
                let current_next_node_new_key = next_possible_node
                    .clone()
                    .strip_prefix(&next_key)
                    .unwrap()
                    .to_owned();

                // Special case where the prefix is also the same as the new word inserted.
                // in this case terminate early.
                let is_new_node_terminal = next_key == current_word;

                let mut new_next_node = RadixTrieNode {
                    children: HashMap::new(),
                    is_terminal: is_new_node_terminal,
                };
                new_next_node
                    .children
                    .insert(current_next_node_new_key, current_next_node.to_owned());

                current
                    .children
                    .insert(next_key.clone(), new_next_node.to_owned());

                current_word = &current_word[next_key.len()..];
                current = current.children.get_mut(&next_key).unwrap();
            }
        }
    }

    pub fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }
        let mut current_node = &self.root;
        let mut word_part = word;

        while !word_part.is_empty() {
            let next_node = current_node
                .children
                .iter()
                .find(|(key, _)| word_part.starts_with(*key));

            if let Some((key, node)) = next_node {
                word_part = &word_part[key.len()..];
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_terminal
    }

    pub fn delete(&mut self, word: &str) {
        if !word.is_empty() {
            recursively_delete_node(&mut self.root, word);
        }
    }
}

fn get_common_prefix<'a>(word_a: &'a str, word_b: &'a str) -> &'a str {
    let mut end = 0;

    if word_a.is_empty() || word_b.is_empty() {
        return "";
    }

    for (a, b) in word_a.chars().zip(word_b.chars()) {
        if a == b {
            end += a.len_utf8();
        } else {
            break;
        }
    }
    &word_a[..end]
}

fn recursively_delete_node(node: &mut RadixTrieNode, word: &str) -> bool {
    if word.is_empty() && node.is_terminal {
        node.is_terminal = false;
        if !node.children.is_empty() {
            return false;
        }
        return true;
    }

    // Find the child key that is a prefix of the word to follow the path.
    let next_key = node
        .children
        .keys()
        .find(|&child_key| word.starts_with(child_key))
        .cloned();

    // if we cannot find the next key, it means the word doesn't exist in the tree.
    if let Some(next_key) = next_key {
        let next_word = &word[next_key.len()..];

        let should_delete_node = {
            let next_node = node.children.get_mut(&next_key).unwrap();
            recursively_delete_node(next_node, next_word)
        };

        if should_delete_node {
            node.children.remove(&next_key);

            // If there's a non terminal leaf, it should be deleted too.
            if node.children.is_empty() && !node.is_terminal {
                return true;
            }
        } else {
            let mut child_node = node.children.remove(&next_key).unwrap();

            // Try to compress the child node.
            let actual_next_key = if !child_node.is_terminal && child_node.children.len() == 1 {
                let children_clone = child_node.children.clone();
                // This definitely exist, since there's only a single child.
                let child = children_clone.iter().next().unwrap();
                // This should always work too, given all child should have value.
                child_node.children = child.1.children.clone();
                child_node.is_terminal = child.1.is_terminal;

                format!("{}{}", next_key, child.0)
            } else {
                next_key.clone()
            };

            if actual_next_key != next_key {
                node.children.remove(&next_key);
            }

            node.children.insert(actual_next_key, child_node);
        }
    }

    false
}

fn visualize_trie(node: &RadixTrieNode, label: &str, prefix: &str, is_last: bool) {
    // Print the current node
    let marker = if is_last { "└── " } else { "├── " };
    let value = if label.is_empty() { "ROOT" } else { label };
    let terminal = if node.is_terminal { " (T)" } else { "" };
    println!("{prefix}{marker}{value}{terminal}");

    // Calculate the new prefix for children
    let new_prefix = if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };

    // Sort children for consistent visualization
    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by_key(|(c, _)| *c);

    // Print children
    for (i, (child_key, child)) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        visualize_trie(child, child_key, &new_prefix, is_last_child);
    }
}

fn main() {
    let mut trie = RadixTrie::new();
    trie.insert("hello");
    trie.insert("hell");
    trie.insert("world");
    trie.insert("hi");
    trie.insert("wow");
    trie.insert("win");

    println!("Trie Structure:");
    visualize_trie(&trie.root, "", "", true);

    println!("{:?}", trie.search("hello"));
    println!("{:?}", trie.search("hell"));
    println!("{:?}", trie.search("world"));
    println!("{:?}", trie.search("hi"));
    println!("{:?}", trie.search("win"));
    trie.delete("hell");
    trie.delete("hello");
    println!("{:?}", trie.search("hello"));
    println!("{:?}", trie.search("hell"));

    println!("Trie Structure after deletion:");
    visualize_trie(&trie.root, "", "", true);

    trie.delete("hello");
    println!("Trie Structure after deletion:");
    visualize_trie(&trie.root, "", "", true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common_prefix() {
        let string_a = "abcdefg";
        let string_b = "abcdt";
        let result = get_common_prefix(string_a, string_b);
        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_get_common_prefix_no_prefix() {
        let string_a = "abcdefg";
        let string_b = "dbcdef";
        let result = get_common_prefix(string_a, string_b);
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_common_prefix_full_match() {
        let string_a = "abcdefg";
        let string_b = "abcdefg";
        let result = get_common_prefix(string_a, string_b);
        assert_eq!(result, "abcdefg");
    }

    #[test]
    fn test_delete_and_compress_logic() {
        let mut trie = RadixTrie::new();
        trie.insert("testing");
        trie.insert("tester");

        trie.delete("tester");

        assert!(trie.search("testing"));
        assert!(!trie.search("tester"));
    }

    #[test]
    fn test_delete_prefix_of_existing_word() {
        let mut trie = RadixTrie::new();
        trie.insert("hello");
        trie.insert("hell");

        trie.delete("hell");
        assert!(!trie.search("hell"));
        assert!(trie.search("hello"));

        // This should not panic and should not delete "hello"
        trie.delete("he");
        assert!(trie.search("hello"));
    }
}
