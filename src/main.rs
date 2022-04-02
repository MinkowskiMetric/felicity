use std::collections::HashMap;

use felicity::heap::{Heap, MinHeap};

#[derive(Debug)]
enum Symbol {
    Leaf { frequency: usize, symbol: char },
    Internal { frequency: usize, left: Box<Self>, right: Box<Self>, },
}

#[derive(Debug, Clone)]
struct Code {
    symbol: char,
    code: String,       // There is probably a better way to store the bit sequence :)
}

impl Symbol {
    fn frequency(&self) -> usize {
        match self {
            Self::Leaf { frequency, .. } |
            Self::Internal { frequency, .. } => *frequency,
        }
    }

    fn build_codes(&self) -> HashMap<char, String> {
        let mut codes = HashMap::new();
        self.build_codes_with_prefix("", &mut codes);
        codes
    }

    fn build_codes_with_prefix(&self, prefix: &str, codes: &mut HashMap<char, String>) {
        match self {
            Self::Leaf { symbol, .. } => {
                codes.insert(*symbol, prefix.to_string());
            }
            Self::Internal { left, right, ..} => {
                left.build_codes_with_prefix(&format!("{}0", prefix), codes);
                right.build_codes_with_prefix(&format!("{}1", prefix), codes);
            }
        }
    }
}

impl std::cmp::PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl std::cmp::Eq for Symbol { }

impl std::cmp::PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.frequency().partial_cmp(&other.frequency())
    }
}

impl std::cmp::Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency().cmp(&other.frequency())
    }
}

fn main() {
    let string = "Hello, world!";

    // Start by collecting the frequencies of the symbols
    let mut frequencies = HashMap::new();

    for symbol in string.chars() {
        *frequencies.entry(symbol).or_insert(0) += 1;
    }

    println!("{}: {:#?}", string, frequencies);

    let mut code_heap = MinHeap::with_capacity((2*frequencies.len())-1);
    code_heap.extend(frequencies.into_iter().map(|(symbol, frequency)| Box::new(Symbol::Leaf { frequency, symbol, })));

    while code_heap.len() > 1 {
        let left = code_heap.remove(0);
        let right = code_heap.remove(0);

        let combined_node = Box::new(Symbol::Internal {
            frequency: left.frequency() + right.frequency(),
            left,
            right,
        });

        code_heap.insert(combined_node);
    }

    let root = code_heap.remove(0);
    let codes = root.build_codes();

    println!("{:#?}", codes);

    let encoded: String = string.chars().map(|char| codes.get(&char).unwrap().to_string()).collect();
    println!("{}", encoded);
}
