// Implementing a radix trie

// Patricia trie is a binary radix trie, with bits as keys instead of string.
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PatriciaTrieNode {
    is_terminal: bool,
    value: Option<String>,
    children: HashMap<String, PatriciaTrieNode>,
}

struct PatriciaTrie {
    root: PatriciaTrieNode,
}

// impl PatriciaTrie {
//     fn new() -> Self {
//         Self {
//             root: PatriciaTrieNode {
//                 is_terminal: false,
//                 value: None,
//                 children: HashMap::new(),
//             },
//         }
//     }

//     fn insert(&mut self, input_word: &str) {
//         let mut terminal_reached = false;

//         let mut current: &mut PatriciaTrieNode = &mut self.root;
//         let mut word: String = input_word.to_owned();
//         while !terminal_reached {
//             // Find common prefix and potential next node.

//             let next_keys = current.children.keys().find_map(|child_key| {
//                 let common_prefix = get_common_prefix(&word, child_key);
//                 if !common_prefix.is_empty() {
//                     return Some((child_key.to_owned(), common_prefix));
//                 }
//                 None
//             });

//             // With the common prefix and the next node known, we now go through the cases.]
//             // 1. if no common prefix, insert the node as it is /
//             // 2. if common prefix == input word length, return immediately (node already exists)
//             // 3. if common prefix length == value length of the nodes, continue down (continue searching)
//             // 4. if common prefix not equals, then split the node into its common prefix and its postfix(reorg).
//             let (next_possible_node, next_key) = match next_keys {
//                 None => {
//                     // Case 1: no common prefix, insert new word whole from root.
//                     current.children.insert(
//                         word.clone(),
//                         PatriciaTrieNode {
//                             is_terminal: true,
//                             value: Some(word.to_string()),
//                             children: HashMap::new(),
//                         },
//                     );
//                     break;
//                 }
//                 Some((next_possible_node, next_key)) => (next_possible_node, next_key),
//             };

//             if next_possible_node == word {
//                 // Case 2: common prefix exists as a node, and its the same as the input, thus the node already exists, and we don't insert.
//                 terminal_reached = true;
//             } else if next_key == next_possible_node {
//                 // Case 3: there's already a node of the prefix, so we continue searching
//                 let next_node = current.children.get_mut(&next_possible_node).unwrap();
//                 word = word.strip_prefix(&next_possible_node).unwrap().to_owned();
//                 current = next_node;
//             } else {
//                 // Case 4: There's a common prefix, and an existing node, we split the node and reorg the tree.
//                 let mut current_next_node = current.children.remove(&next_possible_node).unwrap();
//                 let current_next_node_new_key = current_next_node
//                     .value
//                     .clone()
//                     .unwrap()
//                     .strip_prefix(&next_key)
//                     .unwrap()
//                     .to_owned();
//                 current_next_node.value = Some(current_next_node_new_key.clone());

//                 // Special case where the prefix is also the same as the new word inserted.
//                 // in this case terminate early.
//                 let is_new_node_terminal = next_key == word;

//                 let mut new_next_node = PatriciaTrieNode {
//                     children: HashMap::new(),
//                     is_terminal: is_new_node_terminal,
//                     value: Some(next_key.clone()),
//                 };
//                 new_next_node
//                     .children
//                     .insert(current_next_node_new_key, current_next_node.to_owned());

//                 current
//                     .children
//                     .insert(next_key.clone(), new_next_node.to_owned());

//                 word = word.strip_prefix(&next_key).unwrap().to_owned();
//                 current = current.children.get_mut(&next_key).unwrap();

//                 terminal_reached = is_new_node_terminal;
//             }
//         }
//     }

//     fn search(&self, word: &str) -> bool {
//         let mut terminal_reached = false;
//         let mut current = &self.root;
//         let mut current_word = word.to_owned();

//         while !terminal_reached {
//             let next_key = current.children.keys().find_map(|child_key| {
//                 let common_prefix = get_common_prefix(&current_word, child_key);
//                 if !common_prefix.is_empty() {
//                     return Some(common_prefix);
//                 }
//                 None
//             });

//             let next_key = match next_key {
//                 None => {
//                     return false;
//                 }
//                 Some(key) => key,
//             };

//             let next_val = current.children.get(&next_key);
//             match next_val {
//                 None => return false,
//                 Some(val) => {
//                     if val.value == Some(current_word.clone()) && val.is_terminal {
//                         terminal_reached = true
//                     } else {
//                         // Unwrapping here since there's no chance stripping will fail
//                         current_word = current_word.strip_prefix(&next_key).unwrap().to_owned();
//                     }
//                     current = val;
//                 }
//             }
//         }

//         current.is_terminal
//     }

//     fn delete(&mut self, word: &str) {
//         recursively_delete_node(&mut self.root, word);
//     }
// }

// fn get_common_prefix(word_a: &str, word_b: &str) -> String {
//     let mut common_prefix = String::new();

//     if word_a.is_empty() || word_b.is_empty() {
//         return common_prefix;
//     }

//     let word_a_vec = word_a.chars().collect::<Vec<char>>();
//     let word_b_vec = word_b.chars().collect::<Vec<char>>();

//     let (long_vec, short_vec) = if word_a_vec.len() > word_b.len() {
//         (word_a_vec, word_b_vec)
//     } else {
//         (word_b_vec, word_a_vec)
//     };

//     for (idx, c) in short_vec.into_iter().enumerate() {
//         if c != long_vec[idx] {
//             break;
//         }
//         common_prefix.push(c);
//     }

//     common_prefix
// }

// fn recursively_delete_node(node: &mut PatriciaTrieNode, word: &str) -> Option<PatriciaTrieNode> {
//     if word.is_empty() && node.is_terminal {
//         node.is_terminal = false;
//         if !node.children.is_empty() {
//             // Special case: if one node children, we consolidate
//             if node.children.len() == 1 {
//                 let children_clone = node.children.clone();
//                 // This definitely exist, since there's only a single child.
//                 let child = children_clone.iter().next().unwrap();
//                 // This should always work too, given all child should have value.
//                 node.value.as_mut().unwrap().push_str(child.0);
//                 node.children = child.1.children.clone();
//                 node.is_terminal = child.1.is_terminal;
//             }
//             return Some(node.clone());
//         }
//         node.value = None;
//         return None;
//     }
//     // Recursively delete the node
//     let next_key = node
//         .children
//         .keys()
//         .find_map(|child_key: &String| {
//             let common_prefix = get_common_prefix(word, child_key);
//             if !common_prefix.is_empty() {
//                 return Some(common_prefix);
//             }
//             None
//         })
//         .unwrap();

//     let next_node = node.children.get_mut(&next_key).unwrap();
//     let next_word = &word[next_key.len()..];
//     let new_node = recursively_delete_node(next_node, next_word);
//     match new_node {
//         None => {
//             node.children.remove(&next_key);
//             if node.children.is_empty() {
//                 node.value = None;
//                 return None;
//             }
//         }
//         Some(trie_node) => {
//             let mut actual_next_key = next_key.clone();
//             // Special case, if node only a single one we can reconsolidate
//             let trie_node_actual_key = trie_node.value.clone().unwrap();
//             if trie_node_actual_key != next_key {
//                 node.children.remove(&next_key);
//                 actual_next_key = trie_node_actual_key;
//             }

//             node.children.insert(actual_next_key, trie_node);
//         }
//     };

//     Some(node.clone())
// }

// fn visualize_trie(node: &PatriciaTrieNode, prefix: &str, is_last: bool) {
//     // Print the current node
//     let marker = if is_last { "└── " } else { "├── " };
//     let value = node
//         .value
//         .clone()
//         .map_or("ROOT".to_string(), |c| c.to_string());
//     let terminal = if node.is_terminal { " (T)" } else { "" };
//     println!("{}{}{}{}", prefix, marker, value, terminal);

//     // Calculate the new prefix for children
//     let new_prefix = if is_last {
//         format!("{}    ", prefix)
//     } else {
//         format!("{}│   ", prefix)
//     };

//     // Sort children for consistent visualization
//     let mut children: Vec<_> = node.children.iter().collect();
//     children.sort_by_key(|(c, _)| *c);

//     // Print children
//     for (i, (_, child)) in children.iter().enumerate() {
//         let is_last_child = i == children.len() - 1;
//         visualize_trie(child, &new_prefix, is_last_child);
//     }
// }

fn main() {
    //     let mut trie = PatriciaTrie::new();
    //     trie.insert("hello");
    //     trie.insert("hell");
    //     trie.insert("world");
    //     trie.insert("hi");
    //     trie.insert("wow");
    //     trie.insert("win");

    //     println!("Trie Structure:");
    //     visualize_trie(&trie.root, "", true);

    //     println!("{:?}", trie.search("hello"));
    //     println!("{:?}", trie.search("hell"));
    //     println!("{:?}", trie.search("world"));
    //     println!("{:?}", trie.search("hi"));
    //     println!("{:?}", trie.search("win"));
    //     trie.delete("hell");
    //     trie.delete("hello");
    //     println!("{:?}", trie.search("hello"));
    //     println!("{:?}", trie.search("hell"));

    //     println!("Trie Structure after deletion:");
    //     visualize_trie(&trie.root, "", true);

    //     trie.delete("hello");
    //     println!("Trie Structure after deletion:");
    //     visualize_trie(&trie.root, "", true);
}
