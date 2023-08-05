use crate::test_state::Key;
use std::io::{self, Write};

use termion::event::Key as TermionKey;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

pub struct IOController {
    pub stdin_keys: termion::input::Keys<io::Stdin>,
    pub stdout: RawTerminal<io::Stdout>,
}

impl IOController {
    pub fn new() -> Self {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let mut keys = io::stdin().keys();
        IOController {
            stdout,
            stdin_keys: keys,
        }
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

    pub fn clear(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1));
    }
}
