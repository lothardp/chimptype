use std::cmp::max;
use std::io::{Read, Write};
use std::time::Duration;
use std::time::Instant;
use termion::event::Key as TermionKey;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod test_state;
use test_state::{Key, TestState};

enum Command {
    StartTest,
    Exit,
}

#[derive(Debug)]
struct TestResult {
    word_list: Vec<String>,
    duration: Duration,
    final_state: TestState,
}

#[derive(Debug)]
struct WordResult {
    word: String,
    typed_word: Vec<Key>,
    correct: u32,
    incorrect: u32,
    duration: std::time::Duration,
}

fn main() {
    loop {
        welcome_message();
        let test_result = execute_test();
        if let Ok(test_result) = test_result {
            finish_test(&test_result);
        } else {
            println!("Test aborted");
        }
        /* match get_command() {
            Command::StartTest => {
                println!("Starting test");
                let test_result = execute_test();
                println!("Test complete");
            }
            Command::Exit => {
                println!("Exiting");
                return;
            }
        } */
    }
}

fn welcome_message() {
    loop {
        println!("Welcome to the typing test");
        println!("Press enter to start the test");
        println!("Press esc or q to exit");
        match read_one_char(&mut std::io::stdin()) {
            Key::Enter => break,
            Key::Char('q') | Key::Char('Q') | Key::Esc => {
                println!("Exiting");
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn show_results(test_result: &TestResult) {
    println!("Results:");
    println!("Duration: {:?}", test_result.duration);
    println!("WPM: {:.1}", calculate_wpm(test_result));
}

fn calculate_wpm(test_result: &TestResult) -> f32 {
    let duration_in_seconds = test_result.duration.as_secs_f32();
    let minutes = duration_in_seconds / 60.0;
    let wpm = test_result.word_list.len() as f32 / minutes;
    wpm
}

fn execute_test() -> Result<TestResult, ()> {
    let word_list = generate_word_list();
    show_word_list(&word_list);
    run_test(word_list)
}

fn run_test(word_list: Vec<String>) -> Result<TestResult, ()> {
    let mut test_state = TestState::new(&word_list);
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = std::io::stdin();
    write!(stdout, "{}", clear::All).unwrap();
    draw(&mut stdout, &test_state);

    while !test_state.finished {
        let key = read_one_char(&mut stdin);
        test_state.handle_key(key)?;
        draw(&mut stdout, &test_state);
    }

    Ok(TestResult {
        word_list,
        duration: Duration::from_secs(0),
        final_state: test_state,
    })
}

fn draw<W: Write>(stdout: &mut W, test_state: &TestState) {
    let PADDING = 30;
    let (columns, rows) = termion::terminal_size().unwrap();
    let width = columns - PADDING * 2;
    let base = rows / 2 - 3;
    let (col, mut row, mut written) = (PADDING, base, 0);
    write!(stdout, "{}", cursor::Goto(col, row)).unwrap();

    let (words, typed_words) = (&test_state.word_list, &test_state.typed_words());
    let mut word_i = 0;
    while word_i < words.len() {
        let word = words.get(word_i).unwrap();
        let empty_word = &Vec::new();
        let typed_word = typed_words.get(word_i).unwrap_or(empty_word);
        if written + max(word.len(), typed_word.len()) >= width.into() {
            written = 0;
            row += 1;
            write!(stdout, "{}", cursor::Goto(col, row)).unwrap();
        }
        write_word(stdout, word, typed_word);
        write!(stdout, " ").unwrap();
        word_i += 1;
    }
    stdout.flush().unwrap();
}

fn write_word<W: Write>(stdout: &mut W, word: &[Key], typed_word: &[Key]) {
    let mut i = 0;
    loop {
        let word_char = word.get(i);
        let typed_char = typed_word.get(i);
        match (word_char, typed_char) {
            (Some(Key::Char(word_char)), Some(Key::Char(typed_char))) => {
                if word_char == typed_char {
                    write!(stdout, "{}", termion::style::Bold).unwrap();
                    write!(stdout, "{}", termion::color::Fg(termion::color::Green),).unwrap();
                } else {
                    write!(stdout, "{}", termion::color::Fg(termion::color::Red),).unwrap();
                }
                write!(stdout, "{}", typed_char).unwrap();
                write!(stdout, "{}", termion::style::Reset).unwrap();
            }
            (Some(Key::Char(word_char)), None) => {
                write!(stdout, "{}", word_char).unwrap();
            }
            (None, Some(Key::Char(typed_char))) => {
                write!(stdout, "{}", termion::color::Fg(termion::color::Red)).unwrap();
                write!(stdout, "{}", termion::style::Underline).unwrap();
                write!(stdout, "{}", typed_char).unwrap();
                write!(stdout, "{}", termion::style::Reset).unwrap();
            }
            (None, None) => break,
            _ => {
                unreachable!("Only chars should get here")
            }
        }
        i += 1;
    }
}

fn finish_test(test_result: &TestResult) {
    println!("Test complete");
    show_results(test_result);
}


fn read_one_char<R: Read>(stdin: &mut R) -> Key {
    match stdin.keys().next() {
        Some(Ok(TermionKey::Char(ch))) if ch == ' ' => Key::Space,
        Some(Ok(TermionKey::Char(ch))) if ch == '\n' => Key::Enter,
        Some(Ok(TermionKey::Char(ch))) => Key::Char(ch),
        Some(Ok(TermionKey::Backspace)) => Key::Backspace,
        Some(Ok(TermionKey::Esc)) => Key::Esc,
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
