use crate::test_state::Key;
use std::io::{self, Write};

use termion::clear;
use termion::cursor;
use termion::event::Key as TermionKey;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct IOController {
    pub stdin_keys: termion::input::Keys<io::Stdin>,
    pub stdout: RawTerminal<io::Stdout>,
}

// Only public methods flush the stdout
impl IOController {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdin_keys = io::stdin().keys();
        IOController { stdout, stdin_keys }
    }

    pub fn read_one_char(&mut self) -> Key {
        match self.stdin_keys.next() {
            Some(Ok(TermionKey::Char(ch))) if ch == ' ' => Key::Space,
            Some(Ok(TermionKey::Char(ch))) if ch == '\n' => Key::Enter,
            Some(Ok(TermionKey::Char(ch))) => Key::Char(ch),
            Some(Ok(TermionKey::Backspace)) => Key::Backspace,
            Some(Ok(TermionKey::Esc)) => Key::Esc,
            _ => panic!("Error reading a key"),
        }
    }

    // Clear the screen moving everything up. To clear in place
    // use clear_screen()
    pub fn clear_screen_raw(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
        self.flush();
    }

    // Clear the screen in place
    pub fn clear_screen(&mut self) {
        let (_, rows) = termion::terminal_size().unwrap();
        for row in 1..=rows {
            self.clear_row(row);
        }
        self.flush();
    }

    fn clear_row(&mut self, row: u16) {
        write!(self.stdout, "{}", cursor::Goto(1, row)).unwrap();
        write!(self.stdout, "{}", clear::CurrentLine).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
