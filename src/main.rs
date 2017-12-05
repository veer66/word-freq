#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref WORD_RE: Regex = Regex::new("[^\t\r\n \\|\"=\\[\\]]+").unwrap();
}

fn main() {
    let mut map = HashMap::new();
    for line in io::BufReader::new(io::stdin()).lines() {
        let line = line.unwrap();
        for word in WORD_RE.find_iter(&line[..]) {
            let surface = word.as_str().to_string();
            let cnt = map.entry(surface).or_insert(0);
            *cnt += 1;
        }
    }
    let mut word_freq: Vec<_> = map.into_iter().collect();
    word_freq.sort_by_key(|i| i.1);
    word_freq.reverse();

    for (word, freq) in word_freq {
        println!("{}\t{}", word, freq);
    }
}
