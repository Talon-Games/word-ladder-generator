pub mod display;

use display::string_input::StringInput;
use std::collections::{HashSet, VecDeque};

fn main() {
    let starting_word = StringInput::new()
        .message("Enter starting word")
        .min(3)
        .max(10)
        .ask();

    let ending_word = StringInput::new()
        .message("Enter ending word")
        .min(starting_word.len() as i32)
        .max(starting_word.len() as i32)
        .ask();

    if starting_word == ending_word {
        eprintln!("The starting word must be different from the ending word");
        std::process::exit(1);
    }

    let skip_word = StringInput::new().message("Enter words to skip").ask();

    let words_to_skip: Vec<&str> = skip_word.split(",").collect();

    println!("{} -> {}", starting_word, ending_word);

    let file = include_str!("./words.txt");
    let mut word_list: Vec<String> = Vec::new();

    let mut has_start = false;
    let mut has_end = false;
    for line in file.lines() {
        if line.len() != starting_word.len() {
            continue;
        }

        if words_to_skip.contains(&line) {
            continue;
        }

        if line == starting_word {
            has_start = true;
        }

        if line == ending_word {
            has_end = true;
        }

        word_list.push(line.to_string());
    }

    if has_start == false {
        eprintln!("Failed to find start word in word list");
        std::process::exit(1);
    }

    if has_end == false {
        eprintln!("Failed to find end word in word list");
        std::process::exit(1);
    }

    let ladder = generate_word_ladder(starting_word, ending_word, word_list);

    if ladder.is_empty() {
        println!("Failed to find path between words");
        return;
    }

    for word in ladder {
        println!("{}", word);
    }
}

fn find_successors(word: &str, word_set: &HashSet<String>) -> Vec<String> {
    let mut successors = Vec::new();
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..word_chars.len() {
        for ch in 'a'..='z' {
            if ch != word_chars[i] {
                let mut new_word = word_chars.clone();
                new_word[i] = ch;
                let candidate: String = new_word.into_iter().collect();

                if word_set.contains(&candidate) {
                    successors.push(candidate);
                }
            }
        }
    }
    successors
}

fn generate_word_ladder(start: String, end: String, word_list: Vec<String>) -> Vec<String> {
    let word_set: HashSet<String> = word_list.iter().cloned().collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(vec![start.clone()]);
    visited.insert(start.clone());

    while let Some(ladder) = queue.pop_front() {
        let last_word = ladder.last().unwrap();

        if last_word == &end {
            return ladder;
        }

        for successor in find_successors(last_word, &word_set) {
            if !visited.contains(&successor) {
                let mut new_ladder = ladder.clone();
                new_ladder.push(successor.clone());
                queue.push_back(new_ladder);
                visited.insert(successor);
            }
        }
    }

    Vec::new()
}
