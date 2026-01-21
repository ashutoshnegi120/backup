use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_freq = frequency_map(&word_lower);

    let mut result = HashSet::new();

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if candidate_lower == word_lower {
            continue;
        }

        if frequency_map(&candidate_lower) == word_freq {
            result.insert(candidate);
        }
    }

    result
}

fn frequency_map(s: &str) -> HashMap<char, i32> {
    let mut freq = HashMap::new();

    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    freq
}

