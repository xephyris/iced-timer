use std::time::{Instant, Duration};

#[derive(Default)]
pub struct Stopwatch {
    start: Option<Instant>,
    duration: Duration,
    time_str: String,
}

impl Stopwatch {

    pub fn new() -> Self {
        Stopwatch {
            start: None,
            duration: Duration::ZERO,
            time_str: Stopwatch::dur_to_string(Duration::ZERO, true)
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn started(&self) -> bool {
        if self.start.is_some() {
            true
        } else {
            false
        }
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.start {
            self.duration = self.duration + start.elapsed();
            self.start = None;
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.duration = Duration::ZERO;
        self.time_str = Stopwatch::dur_to_string(Duration::ZERO, false)
    }

    pub fn toggle(&mut self, save: bool) {
        if let Some(_start) = self.start {
            println!("Stopping");
            self.pause();
        } else {
            println!("Starting");
            self.start();
        }
    }

    pub fn tick(&mut self) {
        if let Some(start) = self.start {
            self.time_str = Stopwatch::dur_to_string(self.duration + start.elapsed(), true);
        } else {
            // println!("Stopwatch has not been started");
        }
    }


    fn dur_to_string(duration: Duration, with_ms: bool) -> String {
        let hours = duration.as_secs() / 3600;
        let minutes = duration.as_secs() % 3600 / 60;
        let seconds = duration.as_secs() % 60;
        if hours < 1 && with_ms {
            let millis = (duration.as_millis() % 1000) as u64 / 10;
            format!("{:02}:{:02}.{:02}", minutes, seconds, millis)
        } else {
            // println!("{}:{:02}:{:02}", hours, minutes, seconds);
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        }
    }

    pub fn ms_enabled(&self) -> bool {
        if let Some(start) = self.start {
            if (self.duration + start.elapsed()).as_secs() / 3600 < 1{
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        self.time_str.clone()
    }

    pub fn to_string_ms_removed(&self) -> String {
        self.time_str.split(".").next().unwrap_or("00:00").to_string()
    }

    pub fn to_hmsms(&self) -> (String, String, String) {
        let segments: Vec<String> = self.time_str.split(":").map(|x| x.to_owned()).collect();
        (segments[0].to_owned(), segments[1].to_owned(), segments[2].to_owned())
    }
}