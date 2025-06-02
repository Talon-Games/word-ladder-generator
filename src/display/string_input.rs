use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::display::refresh_display;

pub struct StringInput {
    message: String,
    min: i32,
    max: Option<i32>,
    pub full_size: i32,
    reset_size: i32,
    manual_clear: bool,
}

impl StringInput {
    pub fn new() -> Self {
        StringInput {
            message: String::new(),
            min: 0,
            max: None,
            full_size: 3,
            reset_size: 1,
            manual_clear: false,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn ask(&self) -> String {
        println!("{}: ", self.message);

        if self.min != 0 || self.max.is_some() {
            if self.max.is_none() {
                println!("String must be at least {} chars", self.min);
            } else {
                if self.min == self.max.unwrap() {
                    println!("String must be {} chars", self.min,);
                } else {
                    println!(
                        "String must be between {} and {} chars",
                        self.min,
                        self.max.unwrap()
                    );
                }
            }
        }

        let mut current_string = String::new();

        loop {
            println!("> {}", current_string);
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char(c) => {
                        if c.is_alphabetic() || c == ',' {
                            current_string.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        current_string.pop();
                    }
                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if current_string.len() >= self.min as usize {
                            if let Some(max) = self.max {
                                if current_string.len() <= max as usize {
                                    if !self.manual_clear {
                                        refresh_display(self.full_size);
                                    }
                                    return current_string;
                                }
                            } else {
                                if !self.manual_clear {
                                    refresh_display(self.full_size);
                                }
                                return current_string;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(self.reset_size);
        }
    }
}
