use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryMode {
  Off,
  Standard,
  Adaptive,
}

impl Default for RetryMode {
  fn default() -> Self {
    RetryMode::Standard
  }
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
  pub mode: RetryMode,
  pub max_attempts: u32,
  pub initial_delay: Duration,
  pub max_delay: Duration,
  pub backoff_multiplier: f64,
}

impl RetryConfig {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_mode(mut self, mode: RetryMode) -> Self {
    self.mode = mode;
    self
  }

  pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
    self.max_attempts = max_attempts;
    self
  }

  pub fn with_initial_delay(mut self, delay: Duration) -> Self {
    self.initial_delay = delay;
    self
  }

  pub fn with_max_delay(mut self, delay: Duration) -> Self {
    self.max_delay = delay;
    self
  }

  pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
    if self.mode == RetryMode::Off {
      return Duration::from_secs(0);
    }

    let delay_secs = self.initial_delay.as_secs_f64()
      * self.backoff_multiplier.powi(attempt as i32);

    let delay = Duration::from_secs_f64(delay_secs);

    if delay > self.max_delay {
      self.max_delay
    } else {
      delay
    }
  }
}

impl Default for RetryConfig {
  fn default() -> Self {
    Self {
      mode: RetryMode::Standard,
      max_attempts: 3,
      initial_delay: Duration::from_millis(100),
      max_delay: Duration::from_secs(30),
      backoff_multiplier: 2.0,
    }
  }
}