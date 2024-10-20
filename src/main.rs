use chrono::{TimeZone, Utc};
use csv::Writer;
use serde::Serialize;
use std::io;
use std::time::{Instant, SystemTime};

fn main() -> Result<(), io::Error> {
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
                    "clear" => recorder.clear(),
                    "stop" => recorder.stop_recording(),
                    "show" => recorder.show_records(),
                    "persist" => persist(&recorder.records)?,
                    "total" => recorder.get_total(),
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

    return Ok(());
}

fn persist(records: &Vec<Record>) -> Result<(), io::Error> {
    let mut writer = Writer::from_path("test.csv")?;

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;

    println!("Records written to csv");

    return Ok(());
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

#[derive(Debug, Serialize)]
struct Record {
    task: String,
    duration: u64,
    timestamp: String,
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

    fn clear(&mut self) {
        self.records = Vec::new()
    }

    fn start_recording(&mut self, task: String) {
        self.timer.start();
        self.current_task = Some(task);
        self.current_ts = Some(self.get_current_ts());
    }

    fn stop_recording(&mut self) {
        if let Some(task) = self.current_task.take() {
            let duration = self.timer.stop();
            if let Some(seconds) = self.current_ts {
                let timestamp = Utc.timestamp_opt(seconds as i64, 0).unwrap();
                let timestamp = String::from(timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string());
                let record = Record {
                    task,
                    duration,
                    timestamp,
                };
                self.records.push(record);
            }
        }
    }

    fn show_records(&self) {
        println!("{:#?}", self.records);
    }

    fn get_current_ts(&self) -> u64 {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to get timestamp from SystemTime")
            .as_secs();

        return ts;
    }

    fn get_total(&self) {
        let total: f64 = self.records.iter().map(|r| r.duration as f64).sum();
        println!("{} (hours)", total / 60.0 / 60.0);
        return;
    }
}
