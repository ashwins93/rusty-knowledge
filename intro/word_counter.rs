// word_counter.rs

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, val: &u64) {
        let mut keys: Vec<&String> = self.0.keys().collect();
        keys.sort_by(|key1, key2| {
            let count1: u64 = self.0[&key1.to_string()];
            let count2: u64 = self.0[&key2.to_string()];

            if count1 < count2 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        for key in keys {
            let value: u64 = self.0[key];
            if value >= *val {
                println!("{}: {}", key, value);
            }
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    let filter_value: u64 = arguments[2].parse().unwrap();
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display(&filter_value);
}
