use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::Rng;

static DEFAULT_WORD_COUNT : u32 = 30;

fn get_random_text(n: u32) -> String {
    return "random text this is random text".to_string();
} 

fn wait_for_enter() {
    print!("Press Enter to start typing...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    let text = get_random_text(DEFAULT_WORD_COUNT);
    let mut rng = rand::thread_rng();

    loop {
        println!("Type the following text as fast as you can:\n{}", text);

        // Wait for the user to start typing
        wait_for_enter();

        // Measure the time it takes to type the text
        let start_time = Instant::now();
        let mut typed_text = String::new();
        io::stdin().read_line(&mut typed_text).expect("Failed to read line");
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        // Check if the typed text matches the original text
        if typed_text.trim() == text {
            println!("Congratulations! You typed the text correctly in {:.2} seconds.", elapsed_time.as_secs_f64());
        } else {
            println!("Sorry, the typed text doesn't match the original text.");
        }

        // Wait for a random amount of time before asking to type again
        let delay = Duration::from_secs(rng.gen_range(1..6));
        println!("Type again in {} seconds.", delay.as_secs());
        std::thread::sleep(delay);
    }
}
