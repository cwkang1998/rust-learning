use criterion::{Criterion, criterion_group, criterion_main};
use learning_impl::trie::radix::RadixTrie;
use std::{
    fs::{self, File},
    hint::black_box,
    io::{BufRead, BufReader},
};

pub fn bench_insert(c: &mut Criterion) {
    c.bench_function("bench insertion", |b| {
        let words = load_words();
        b.iter(move || {
            let mut trie = RadixTrie::new();
            for word in words.clone() {
                trie.insert(black_box(&word));
            }
        })
    });
}

pub fn bench_search(c: &mut Criterion) {
    c.bench_function("bench search", |b| {
        let words = load_words();
        let mut trie = RadixTrie::new();
        for word in words.clone() {
            trie.insert(black_box(&word));
        }
        b.iter(move || {
            for word in words.clone() {
                let found = trie.search(black_box(&word));
                assert!(found);
            }
        })
    });
}

pub fn bench_delete(c: &mut Criterion) {
    c.bench_function("bench delete", |b| {
        let words = load_words();
        let mut trie = RadixTrie::new();
        for word in words.clone() {
            trie.insert(black_box(&word));
        }
        b.iter(move || {
            for word in words.clone() {
                trie.delete(black_box(&word));
            }
        })
    });
}

// Word list comes from the crate https://crates.io/crates/random_word.
fn load_words() -> Vec<String> {
    let file = File::open("benches/radix_benchmark/bench_data.txt")
        .expect("Unable to open the word list file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect()
}

criterion_group!(benches, bench_insert, bench_delete, bench_search);
criterion_main!(benches);
