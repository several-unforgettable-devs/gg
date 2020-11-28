use bevy::prelude::Time;

// A cooldown until the time in seconds
// Believe me, this is much simpler to use than the timer functionality of bevy
#[derive(Clone, Copy, Default)]
pub struct Cooldown {
    pub until_time_seconds: f64,
}

impl Cooldown {
    pub fn over(&self, time: &Time) -> bool {
        time.seconds_since_startup > self.until_time_seconds
    }
    pub fn create(time: &Time, duration_seconds: f64) -> Self {
        Cooldown {
            until_time_seconds: time.seconds_since_startup + duration_seconds,
        }
    }
    pub fn reset(&mut self, time: &Time, duration_seconds: f64) {
        *self = Self::create(time, duration_seconds)
    }
}
