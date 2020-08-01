use std::time::{Duration, Instant};

pub struct Timer {
    /// Last update moment.
    pub last_instant: Instant,
    /// Time duration between current and last update moments.
    pub delta: Duration,
    /// How often we want the timer to go off, or be ready.
    pub period: Duration,
    /// To keep time of how much time until the timer goes off.
    pub countdown: Duration,
    /// It tells if the timer is ready.
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
