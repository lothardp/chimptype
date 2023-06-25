use std::io::{Read, Write};
use std::time::Instant;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

enum Command {
    StartTest,
    Exit,
}

#[derive(Debug)]
enum Char {
    Char(char),
    Backspace,
    Space,
    Enter,
}

#[derive(Debug)]
struct TestResult {
    word_list: Vec<String>,
    correct: u32,
    incorrect: u32,
    duration: std::time::Duration,
}

#[derive(Debug)]
struct WordResult {
    word: String,
    typed_word: Vec<Char>,
    correct: u32,
    incorrect: u32,
    duration: std::time::Duration,
}

fn main() {
    loop {
        match get_command() {
            Command::StartTest => {
                println!("Starting test");
                let test_result = execute_test();
                println!("Test complete");
            }
            Command::Exit => {
                println!("Exiting");
                return;
            }
        }
    }
}

fn show_results(test_result: &TestResult) {
    dbg!(test_result);
}

fn execute_test() -> TestResult {
    let word_list = generate_word_list();
    show_word_list(&word_list);
    let first_char = read_one_char();
    let test_result = run_test(first_char, word_list);
    finish_test(&test_result);
    test_result
}

fn run_test(first_char: Char, word_list: Vec<String>) -> TestResult {
    let start_time = Instant::now();
    let mut word_result_list: Vec<WordResult> = Vec::new();

    word_result_list.push(run_word_test(&word_list[0], Some(first_char)));
    for word in word_list.iter().skip(1) {
        let word_result = run_word_test(&word, None);
        word_result_list.push(word_result);
    }
    let total_duration = start_time.elapsed();
    construct_test_result(word_list, word_result_list, total_duration)
}

fn finish_test(test_result: &TestResult) {
    show_results(test_result);
}

fn construct_test_result(
    word_list: Vec<String>,
    word_result_list: Vec<WordResult>,
    total_duration: std::time::Duration,
) -> TestResult {
    TestResult {
        word_list,
        correct: word_result_list.iter().map(|r| r.correct).sum(),
        incorrect: word_result_list.iter().map(|r| r.incorrect).sum(),
        duration: total_duration,
    }
}

fn run_word_test(word: &str, first_char: Option<Char>) -> WordResult {
    let start_time = Instant::now();
    let mut ch = match first_char {
        Some(ch) => ch,
        None => read_one_char(),
    };
    let word_chars: Vec<char> = word.chars().collect();
    let mut typed_word = Vec::new();
    let (mut i, mut correct, mut incorrect) = (0, 0, 0);
    loop {
        match ch {
            Char::Space | Char::Enter => break,
            Char::Backspace => {
                typed_word.push(ch);
                if i > 0 {
                    i -= 1;
                }
                ch = read_one_char();
            }
            Char::Char(c) => {
                typed_word.push(ch);
                match word_chars.get(i) {
                    Some(word_char) if word_char == &c => correct += 1,
                    _ => incorrect += 1,
                };
                i += 1;
                ch = read_one_char();
            }
        }
    }
    let duration = start_time.elapsed();
    WordResult {
        word: word.to_string(),
        typed_word,
        correct,
        incorrect,
        duration,
    }
}

fn read_one_char() -> Char {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    match std::io::stdin().keys().next() {
        Some(Ok(Key::Char(ch))) if ch == ' ' => {
            stdout.write(b" ").unwrap();
            stdout.flush().unwrap();
            Char::Space
        }
        Some(Ok(Key::Char(ch))) if ch == '\n' => Char::Enter,
        Some(Ok(Key::Char(ch))) => {
            stdout.write(ch.to_string().as_bytes()).unwrap();
            stdout.flush().unwrap();
            Char::Char(ch)
        }
        Some(Ok(Key::Backspace)) => {
            let mut handle = stdout.lock();
            write!(handle, "{}{}", cursor::Left(1), clear::AfterCursor).unwrap();
            handle.flush().unwrap();
            Char::Backspace
        }
        Some(Ok(Key::Esc)) => panic!("Escape key pressed"),
        _ => panic!("Error reading a key"),
    }
}

fn show_word_list(word_list: &[String]) {
    for word in word_list {
        print!("{} ", word);
    }
    print!("\n\r");
}

fn generate_word_list() -> Vec<String> {
    let words: Vec<&str> = vec![
        "dog", "cat", "sun", "book", "tree", "ball", "happy", "red", "car", "run", "bird", "door",
        "baby", "hat", "song", "fish", "pen", "bed", "star", "milk",
    ];
    words.iter().map(|s| s.to_string()).collect()
}

fn get_command() -> Command {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "start" => return Command::StartTest,
                "exit" => return Command::Exit,
                _ => {
                    println!("Invalid command");
                    continue;
                }
            },
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        }
    }
}
