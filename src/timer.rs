use std::time::{Instant, Duration};

pub struct Timer {
    start: Option<Instant>,
    end: Option<Instant>,
    total_duration: Duration,
    passed_duration: Duration,
    end_duration: Duration,
    time_str: String,
    editing: bool,
    temp_values: Option<(String, String, String)>
}

impl Default for Timer {
    fn default() -> Self {
        Timer { 
            start: Default::default(), 
            end: Default::default(), 
            total_duration: Default::default(), 
            passed_duration: Duration::ZERO, 
            end_duration: Duration::ZERO,
            time_str: Timer::dur_to_string(Duration::default(), false),
            editing: false,
            temp_values: None,
        }
    }
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer {
            start: None,
            end: None,
            total_duration: duration,
            passed_duration: Duration::ZERO,
            end_duration: Duration::ZERO,
            time_str: Timer::dur_to_string(duration, false),
            editing: false,
            temp_values: None,
        }
    }

    pub fn start(&mut self) {
        if self.end_duration.as_secs() > 0 {
            self.end = Some(Instant::now());
        } else {
            self.start = Some(Instant::now());
        }
    }

    pub fn started(&self) -> bool {
        if self.start.is_some() {
            true
        } else {
            false
        }
    }

    pub fn ended(&self) -> bool {
        if self.end.is_some() || self.end_duration.as_secs() > 0 {
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start {
            self.passed_duration = self.passed_duration + start.elapsed();
            self.start = None;
        }
        if let Some(end) = self.end {
            self.end_duration = self.end_duration + end.elapsed();
            self.end = None;
        }
    }

    pub fn reset(&mut self, duration: Duration) {
        self.start = None;
        self.end = None;
        self.passed_duration = Duration::ZERO;
        self.end_duration = Duration::ZERO;
        self.total_duration = duration;
        self.time_str = Timer::dur_to_string(duration, false)
    }

    pub fn toggle(&mut self, save: bool, reset: bool) {
        if !self.editing {
            if let Some(_start) = self.start {
                println!("Stopping");
                self.stop();
            } else if let Some(_end) = self.end {
                self.stop();
            } else {
                println!("Starting");
                self.start();
            }

            if reset {
                self.reset(self.total_duration);
            }
        } else {
            self.toggle_editing(save);
        }
    }

    pub fn ms_enabled(&self) -> bool {
        if let Some(start) = self.start {
            if (self.total_duration - (self.passed_duration + start.elapsed())).as_secs() / 3600 < 1{
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn tick(&mut self) {
        if let Some(start) = self.start {
            if (self.passed_duration + start.elapsed()) < self.total_duration {
                self.time_str = Timer::dur_to_string(self.total_duration.saturating_sub(self.passed_duration + start.elapsed()), true);
            } else {
                self.start = None;
                self.end = Some(Instant::now());
            }
        } else if let Some(end) = self.end {
            self.time_str = Timer::dur_to_string((end.elapsed() + self.end_duration), false);
        } else {
            println!("Timer has not been started");
        }
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn toggle_editing(&mut self, save: bool) {
        if self.editing == false {
            self.stop();
            let duration = self.total_duration - self.passed_duration;
            let hours = (duration.as_secs() / 3600).to_string();
            let minutes = (duration.as_secs() % 3600 / 60).to_string();
            let seconds = (duration.as_secs() % 60).to_string();
            self.time_str = format!("{}:{}:{}", hours, minutes, seconds);
            self.temp_values = Some((hours, minutes, seconds));
        } else {
            if let Some((hour, minute, second)) = &self.temp_values && save {
                // println!("hour, minute, second, {} {} {}", hour, minute, second);
                self.reset(Timer::string_to_duration(format!("{}:{}:{}", hour, minute, second)));
            }
        }
        self.editing = !self.editing;
    }

    pub fn get_temp_vals(&self) -> (String, String, String){
        if let Some(vals) = &self.temp_values {
            vals.clone()
        } else {
            ("".to_owned(), "".to_owned(), "".to_owned())
        }
    }

    pub fn set_temp_vals(&mut self, index: u32, value: String) {
        if let Some(vals) = self.temp_values.as_mut() {
            match index {
                0 => {
                    vals.0 = value;
                }
                1 => {
                    vals.1 = value;
                }
                2 => {
                    vals.2 = value;
                }
                _ => {}
            }
        }
    }

    fn dur_to_string(duration: Duration, with_ms: bool) -> String {
        let hours = duration.as_secs() / 3600;
        let minutes = duration.as_secs() % 3600 / 60;
        let seconds = duration.as_secs() % 60;
        if hours < 1 && with_ms {
            let millis = (duration.as_millis() % 1000) as u64 / 10;
            format!("{:02}:{:02}:{:02}", minutes, seconds, millis)
        } else {
            // println!("{}:{:02}:{:02}", hours, minutes, seconds);
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        }
    }

    fn string_to_duration(hms: String) -> Duration {
        let hms: Vec<u64> = hms.split(":").map(|x| x.parse::<u64>().unwrap_or(0)).collect();
        let time = hms[0] * 3600 + hms[1] * 60 + hms[2];
        Duration::from_secs(time)
    }

    pub fn to_string(&self) -> String {
        self.time_str.clone()
    }

    pub fn to_hmsms(&self) -> (String, String, String) {
        let segments: Vec<String> = self.time_str.split(":").map(|x| x.to_owned()).collect();
        (segments[0].to_owned(), segments[1].to_owned(), segments[2].to_owned())
    }
}
