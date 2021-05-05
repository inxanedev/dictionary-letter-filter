use std::{collections::HashSet, path::Path};
use std::fs;
use std::io::{self, BufReader, BufRead};

use crate::filter::{Filterable};

pub fn make_dictionary<T: AsRef<Path>>(filename: T) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename.as_ref())?;
    let reader = BufReader::new(file);

    let mut words: Vec<String> = Vec::new();

    for word in reader.lines() {
        words.push(word?);
    }

    Ok(words)
}

pub fn get_matches(dict: &Vec<String>, filter: &HashSet<char>) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for word in dict {
        if word.matches(&filter) {
            matches.push(word.clone());
        }
    }

    matches
}