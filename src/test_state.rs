use std::cmp::{max, min};
use std::io::Write;
use termion::cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Backspace,
    Space,
    Enter,
    Esc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestState {
    pub word_list: Vec<Vec<Key>>,
    pub word_index: usize,
    pub typed_chars: Vec<Key>,
    pub raw_char_list: Vec<Key>,
    pub finished: bool,
}

impl TestState {
    pub fn new(word_list: &Vec<String>) -> Self {
        Self {
            word_list: Self::create_word_list(word_list),
            word_index: 0,
            typed_chars: Vec::new(),
            raw_char_list: Vec::new(),
            finished: false,
        }
    }

    fn create_word_list(word_list: &Vec<String>) -> Vec<Vec<Key>> {
        word_list
            .iter()
            .map(|word| word.chars().map(Key::Char).collect())
            .collect()
    }

    pub fn handle_key(&mut self, key: Key) -> Result<(), ()> {
        self.raw_char_list.push(key);
        match key {
            Key::Char(_) => self.handle_char(key),
            Key::Space => self.handle_space(),
            Key::Backspace => self.handle_backspace(),
            _ => return Err(()),
        }
        Ok(())
    }

    // It pops the last typed char, and if it's the first char of the word,
    // it decrements word_index (except when it's the first word)
    fn handle_backspace(&mut self) {
        if self.word_index == 0 && self.typed_chars.is_empty() {
            return;
        }
        let char_index = self.char_index();
        if char_index == 0 {
            self.word_index -= 1;
        }
        self.typed_chars.pop();
    }

    // Right now, it only pushes char to typed_chars
    fn handle_char(&mut self, ch: Key) {
        let current_word = self.current_word();
        let current_word_len = current_word.len();
        let char_index = self.char_index();
        let correct_char = if char_index < current_word_len {
            Some(current_word[char_index])
        } else {
            None
        };
        self.typed_chars.push(ch);
        // Some logic could be here, so far we don't need it I think
        // TODO: add final word OK logic to finish test without space
        match correct_char {
            Some(correct_char) if correct_char == ch => {}
            Some(_) => {}
            None => {}
        }
    }

    // It pushes space to typed_chars and goes to the next word
    // if its the last word it sets finished to true
    fn handle_space(&mut self) {
        self.typed_chars.push(Key::Space);
        if self.word_index == self.word_list.len() - 1 {
            self.finished = true;
        } else {
            self.word_index += 1;
        }
    }

    fn char_index(&self) -> usize {
        self.typed_chars
            .split(|ch| *ch == Key::Space)
            .last()
            .unwrap()
            .len()
    }

    fn current_word(&self) -> Vec<Key> {
        match self.word_list.get(self.word_index) {
            Some(word) => word.to_vec(),
            None => unreachable!("word_index is out of bounds!"),
        }
    }

    pub fn typed_words(&self) -> Vec<Vec<Key>> {
        self.typed_chars
            .split(|ch| *ch == Key::Space)
            .map(|word| word.to_vec())
            .collect()
    }

    pub fn draw<W: Write>(&self, stdout: &mut W) {
        let (columns, rows) = termion::terminal_size().unwrap();
        let width = min(columns - 10, 60);
        let padding = (columns - width) / 2;
        let base_row = rows / 2 - 3;
        let (base_col, mut row, mut written) = (padding, base_row, 0);
        write!(stdout, "{}", cursor::Goto(base_col, row)).unwrap();

        let (words, typed_words) = (&self.word_list, &self.typed_words());
        let mut word_i = 0;
        while word_i < words.len() {
            let word = words.get(word_i).unwrap();
            let empty_word = &Vec::new();
            let typed_word = typed_words.get(word_i);
            let to_write = max(word.len(), typed_word.unwrap_or(empty_word).len()) + 1;
            if written + to_write >= width.into() {
                written = 0;
                row += 1;
                write!(stdout, "{}", cursor::Goto(base_col, row)).unwrap();
            }
            let is_current_word = word_i == self.word_index;
            self.write_word(stdout, word, typed_word, is_current_word);
            written += to_write;
            write!(stdout, " ").unwrap();
            word_i += 1;
        }
        stdout.flush().unwrap();
    }

    // TODO: This probably can be written way better with iterators
    fn write_word<W: Write>(
        &self,
        stdout: &mut W,
        word: &[Key],
        typed_word: Option<&Vec<Key>>,
        is_current_word: bool,
    ) {
        match typed_word {
            None => {
                for ch in word {
                    if let Key::Char(ch) = ch {
                        write!(stdout, "{}", ch).unwrap();
                    }
                }
            }
            Some(typed_word) => {
                let mut i = 0;
                loop {
                    let word_char = word.get(i);
                    let typed_char = typed_word.get(i);
                    match (word_char, typed_char) {
                        (Some(Key::Char(word_char)), Some(Key::Char(typed_char))) => {
                            if word_char == typed_char {
                                write!(stdout, "{}", termion::style::Bold).unwrap();
                                write!(stdout, "{}", termion::color::Fg(termion::color::Green),)
                                    .unwrap();
                            } else {
                                write!(stdout, "{}", termion::color::Fg(termion::color::Red),)
                                    .unwrap();
                            }
                            write!(stdout, "{}", typed_char).unwrap();
                            write!(stdout, "{}", termion::style::Reset).unwrap();
                        }
                        (Some(Key::Char(word_char)), None) => {
                            if !is_current_word {
                                write!(stdout, "{}", termion::style::Underline).unwrap();
                            }
                            write!(stdout, "{}", word_char).unwrap();
                            write!(stdout, "{}", termion::style::Reset).unwrap();
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
        }
    }
}
