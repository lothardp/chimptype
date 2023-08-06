use std::io::Write;
use std::time::Duration;

mod test_state;
use test_state::{Key, TestState};
mod io_controller;
use io_controller::IOController;

pub struct ChimpType {
    io_controller: io_controller::IOController,
    state: State,
}

#[derive(Debug)]
struct TestResult {
    duration: Duration,
    final_state: TestState,
}

enum State {
    Welcome,
    RunningTest(TestState),
    TestComplete(TestResult),
    Exit,
}

impl ChimpType {
    pub fn new() -> Self {
        ChimpType {
            io_controller: IOController::new(),
            state: State::Welcome,
        }
    }

    pub fn start(&mut self) {
        self.io_controller.clear_screen_raw();
        self.draw();
        self.main_loop();
    }

    fn main_loop(&mut self) {
        loop {
            let key = self.io_controller.read_one_char();
            self.handle_key(key);
            self.draw();
            if let State::Exit = self.state {
                break;
            }
        }
    }

    fn handle_key(&mut self, key: Key) {
        match self.state {
            State::Welcome if key == Key::Enter => {
                let word_list = self.generate_word_list();
                self.state = State::RunningTest(TestState::new(&word_list));
            }
            State::Welcome if key == Key::Esc => {
                self.state = State::Exit;
            }
            State::RunningTest(_) if key == Key::Esc => {
                self.state = State::Welcome;
            }
            State::RunningTest(ref mut test_state) => {
                test_state.handle_key(key);
                if test_state.finished {
                    self.state = State::TestComplete(TestResult {
                        duration: Duration::from_secs(0),
                        final_state: test_state.clone(),
                    })
                }
            }
            State::TestComplete(_) => {
                self.state = State::Welcome;
            }
            _ => {}
        };
    }

    // It clears the screen, then draws the current state. TODO: It would
    // be better to only draw the parts that changed, but this is
    // simpler for now.
    fn draw(&mut self) {
        self.io_controller.clear_screen();
        match self.state {
            State::Welcome => {
                self.draw_welcome();
            }
            State::RunningTest(ref test_state) => {
                test_state.draw(&mut self.io_controller.stdout);
            }
            State::TestComplete(_) => {
                self.draw_test_complete();
            }
            _ => {}
        }
    }

    fn draw_welcome(&mut self) {
        write!(self.io_controller.stdout, "Welcome to the typing test\n\r").unwrap();
        write!(
            self.io_controller.stdout,
            "Press enter to start the test\n\r"
        )
        .unwrap();
        write!(self.io_controller.stdout, "Press esc or q to exit\n\r").unwrap();
        self.io_controller.flush();
    }

    fn generate_word_list(&self) -> Vec<String> {
        let words: Vec<&str> = vec![
            "dog", "cat", "sun", "book", "tree", "ball", "happy", "red", "car", "run", "bird",
            "door", "baby", "hat", "song", "fish", "pen", "bed", "star", "milk",
        ];
        words.iter().map(|s| s.to_string()).collect()
    }

    fn draw_test_complete(&mut self) {
        write!(self.io_controller.stdout, "Test complete\n\r").unwrap();
        write!(self.io_controller.stdout, "Results:").unwrap();
        if let State::TestComplete(ref test_result) = self.state {
            write!(
                self.io_controller.stdout,
                "Duration: {:?}",
                test_result.duration
            )
            .unwrap();
        }
        self.io_controller.flush();
    }
}
