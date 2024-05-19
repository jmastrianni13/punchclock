use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let mut timer = Timer::new();
    timer.start();

    sleep(Duration::new(2, 0));

    println!("{} seconds elapsed", timer.stop());
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
