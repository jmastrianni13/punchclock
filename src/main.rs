use std::io;
use std::time::Instant;

fn main() {
    println!("timer app");
    let mut timer = Timer::new();
    let mut command = String::new();
    loop {
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        match command.trim() {
            "start" => timer.start(),
            "stop" => {
                let duration = timer.stop();
                println!("{} seconds elapsed", duration);
            }
            "exit" => break,
            _ => println!("unknown command: {}", command.as_str()),
        }
    }
}

struct Timer {
    start_time: Option<Instant>,
    is_on: bool,
}

impl Timer {
    fn new() -> Timer {
        return Timer {
            start_time: None,
            is_on: false,
        };
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_on = true;
        return;
    }

    fn stop(&mut self) -> u64 {
        match self.is_on {
            true => {
                self.is_on = false;
                return self
                    .start_time
                    .expect("could not calculate elapsed time")
                    .elapsed()
                    .as_secs();
            }
            false => {
                println!("timer is not running, returning 0");
                return 0 as u64;
            }
        }
    }
}
