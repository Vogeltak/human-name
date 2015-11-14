use std::ascii::AsciiExt;

const VOWELS: [char; 12] = ['a','e','i','o','u','y','A','E','I','O','U','Y'];

pub fn is_mixed_case(s: &str) -> bool {
    let mut has_lowercase = false;
    let mut has_uppercase = false;

    for c in s.chars() {
        if c.is_uppercase() { has_uppercase = true; };
        if c.is_lowercase() { has_lowercase = true; };
        if has_lowercase && has_uppercase {
            return true
        };
    }

    false
}

pub fn is_capitalized(word: &str) -> bool {
    match word.chars().nth(0) {
        Some(c) => {
            if !c.is_uppercase() {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    word.chars().skip(1).all( |c| c.is_lowercase() || !c.is_alphabetic() )
}

pub fn capitalize(word: &str) -> String {
    let mut capitalize_next = true;
    word.chars().filter_map( |c| {
        let result = if capitalize_next {
            c.to_uppercase().next()
        } else {
            c.to_lowercase().next()
        };

        capitalize_next = !c.is_alphanumeric();

        result
    }).collect()
}

pub fn is_missing_vowels(word: &str) -> bool {
    word.chars().all(|c| !c.is_alphabetic() || (c.is_ascii() && !VOWELS.contains(&c)))
}
