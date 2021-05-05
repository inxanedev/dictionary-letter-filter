use std::collections::HashSet;

pub fn make_filter<T: AsRef<str>>(letters: &T) -> HashSet<char> {
    let mut filter: HashSet<char> = HashSet::new();

    for letter in letters.as_ref().chars() {
        filter.insert(letter);
    }

    filter
}

pub trait Filterable {
    fn matches_filter(&self, filter: &HashSet<char>) -> bool;
}

impl<T> Filterable for T where T: AsRef<str> {
    fn matches_filter(&self, filter: &HashSet<char>) -> bool {
        self.as_ref().chars().all(|c| filter.contains(&c))
    }
}