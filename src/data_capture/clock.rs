use chrono::{ Utc, };

pub trait Clock {
    fn new() -> Self;
    fn elapsed(&mut self) -> std::time::Duration;
}

#[derive(Default)]
pub struct RealClock {}

impl Clock for RealClock {
    fn new() -> Self {
        Self {}
    }
    fn elapsed(&mut self) -> std::time::Duration {
        std::time::Duration::from_millis(Utc::now().timestamp_millis() as u64)
    }
}

pub struct TestClock {
    calls: u64,
}

impl Clock for TestClock {
    fn elapsed(&mut self) -> std::time::Duration {
        self.calls += 1;
        std::time::Duration::from_millis(10 * (self.calls - 1)) 
    }
    fn new() -> Self {
        Default::default()
    }
}

impl std::default::Default for TestClock {
    fn default() -> Self {
        TestClock { calls: 0 as u64 }   
    }
}


