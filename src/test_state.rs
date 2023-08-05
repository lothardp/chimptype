#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Backspace,
    Space,
    Enter,
    Esc,
}

#[derive(Debug)]
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
            .map(|word| word.chars().map(|ch| Key::Char(ch)).collect())
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
        if self.word_index == 0 && self.typed_chars.len() == 0 {
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
}
