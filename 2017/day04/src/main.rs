use itertools::Itertools;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut valid_passphrases_pt1 = 0;
    let mut valid_passphrases_pt2 = 0;

    loop {
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            break;
        }

        if input.trim().len() == 0 {
            break;
        }

        let (valid_equal_phrases, valid_anagrams) = is_valid_passphrase(&input);

        if valid_equal_phrases {
            valid_passphrases_pt1 += 1;
        }
        if valid_anagrams {
            valid_passphrases_pt2 += 1;
        }
    }

    println!("Part 1: {}", valid_passphrases_pt1);
    println!("Part 2: {}", valid_passphrases_pt2);
}

fn is_valid_passphrase(passphrase: &String) -> (bool, bool) {
    let passphrase: Vec<&str> = passphrase.trim().split_whitespace().collect();
    let mut has_anagram = false;

    for pair in passphrase.iter().combinations(2) {
        let word1 = *pair.get(0).unwrap();
        let word2 = *pair.get(1).unwrap();

        if *word1 == *word2 {
            return (false, false);
        }

        if are_anagrams(*word1, *word2) {
            has_anagram = true;
        }
    }

    (true, !has_anagram)
}

fn are_anagrams(word1: &str, word2: &str) -> bool {
    let mut character_map = HashMap::new();
    let split1 = word1.split("");
    let split2 = word2.split("");

    for c in split1 {
        let new_count = match character_map.get(&c) {
            Some(v) => v + 1,
            None => 1,
        };
        character_map.insert(c, new_count);
    }

    for c in split2 {
        let new_count = match character_map.get(&c) {
            Some(v) => v - 1,
            None => return false,
        };
        if new_count == 0 {
            character_map.remove(c);
        } else {
            character_map.insert(c, new_count);
        }
    }

    character_map.is_empty()
}
