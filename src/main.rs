pub mod display;

use display::string_input::StringInput;
use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Word {
    pub word: String,
    pub definition: String,
}

impl Word {
    pub fn new(word: String, definition: String) -> Word {
        Word { word, definition }
    }
}

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

    println!("start: {}", starting_word);
    println!("end: {}", ending_word);

    let file = include_str!("./words.txt");
    let mut word_list: Vec<Word> = Vec::new();

    let mut start: Option<Word> = None;
    let mut end: Option<Word> = None;
    for line in file.lines() {
        let split: Vec<&str> = line.split(" :: ").collect();
        if starting_word == split[0] {
            start = Some(Word::new(
                split[0].trim().to_string(),
                split[1].trim().to_string(),
            ));
        }

        if ending_word == split[0] {
            end = Some(Word::new(
                split[0].trim().to_string(),
                split[1].trim().to_string(),
            ));
        }

        word_list.push(Word::new(
            split[0].trim().to_string(),
            split[1].trim().to_string(),
        ));
    }

    if start.is_none() {
        eprintln!("Failed to find start word in word list");
        std::process::exit(1);
    }

    if end.is_none() {
        eprintln!("Failed to find end word in word list");
        std::process::exit(1);
    }

    let ladder = generate_word_ladder(start.unwrap(), end.unwrap(), word_list);

    if ladder.is_empty() {
        println!("Failed to find path between words");
        return;
    }

    for word in ladder {
        println!("{}             {}", word.word, word.definition);
    }
}

fn generate_word_ladder(start_word: Word, end_word: Word, word_list: Vec<Word>) -> Vec<Word> {
    let mut queue: VecDeque<Vec<Word>> = VecDeque::new();
    let mut visited: HashSet<Word> = HashSet::new();

    queue.push_back(vec![start_word.clone()]);
    visited.insert(start_word);

    while let Some(path) = queue.pop_front() {
        let current_word = path.last().unwrap();

        if current_word.word == end_word.word {
            return path;
        }

        for word in &word_list {
            if differs_by_one_letter(current_word, &word) && !visited.contains(&word) {
                visited.insert(word.clone());
                let mut new_path = path.clone();
                new_path.push(word.clone());
                queue.push_back(new_path);
            }
        }
    }

    Vec::new()
}

fn differs_by_one_letter(word1: &Word, word2: &Word) -> bool {
    if word1.word.len() != word2.word.len() {
        return false;
    }

    let mut diff_count = 0;

    for (c1, c2) in word1.word.chars().zip(word2.word.chars()) {
        if c1 != c2 {
            diff_count += 1;
            if diff_count > 1 {
                return false;
            }
        }
    }

    diff_count == 1
}
