use std::collections::HashSet;

/// Creates a filter from the list of letters.
pub fn make_filter<T: AsRef<str>>(letters: &T) -> HashSet<char> {
    let mut filter: HashSet<char> = HashSet::new();

    for letter in letters.as_ref().chars() {
        filter.insert(letter);
    }

    filter
}

/// Trait for all types that can be filtered
pub trait Filterable {
    /// Returns true if the object matched the filter.
    fn matches_filter(&self, filter: &HashSet<char>) -> bool;
}

/// Implementation of the filter match function for string-like types
impl<T> Filterable for T where T: AsRef<str> {
    fn matches_filter(&self, filter: &HashSet<char>) -> bool {
        self.as_ref().chars().all(|c| filter.contains(&c))
    }
}