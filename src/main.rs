use std::io;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    println!("timer app");
    let mut recorder = Recorder::new();
    let mut command = String::new();
    loop {
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command_parts = command.trim().split(' ').collect::<Vec<_>>();
        match command_parts {
            command_parts if command_parts.len() == 1 => {
                let c = command_parts[0];
                match c {
                    "stop" => recorder.stop_recording(),
                    "show" => recorder.show_records(),
                    "exit" => break,
                    _ => println!("unknown command: {}", command.as_str()),
                }
            }
            command_parts if command_parts.len() == 2 => {
                let (c1, c2) = (command_parts[0], command_parts[1]);
                match c1 {
                    "start" => recorder.start_recording(c2.to_string()),
                    _ => println!("unknown command: {}", command.as_str()),
                }
            }
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

#[derive(Debug)]
struct Record {
    task: String,
    duration: u64,
    timestamp: u64,
}

struct Recorder {
    timer: Timer,
    records: Vec<Record>,
    current_task: Option<String>,
    current_ts: Option<u64>,
}

impl Recorder {
    fn new() -> Recorder {
        return Recorder {
            timer: Timer::new(),
            records: Vec::new(),
            current_task: None,
            current_ts: None,
        };
    }

    fn start_recording(&mut self, task: String) {
        self.timer.start();
        self.current_task = Some(task);
        self.current_ts = Some(self.get_current_ts());
    }

    fn stop_recording(&mut self) {
        if let Some(task) = self.current_task.take() {
            let duration = self.timer.stop();
            let timestamp = self.current_ts.unwrap();
            let record = Record {
                task,
                duration,
                timestamp,
            };
            self.records.push(record);
        }
    }

    fn show_records(&self) {
        println!("{:?}", self.records);
    }

    fn get_current_ts(&self) -> u64 {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to get timestamp from SystemTime")
            .as_secs();

        return ts;
    }
}
