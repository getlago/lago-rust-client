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
    pub(crate) mode: RetryMode,
    pub(crate) max_attempts: u32,
    pub(crate) initial_delay: Duration,
    pub(crate) max_delay: Duration,
    pub(crate) backoff_multiplier: f64,
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> RetryConfigBuilder {
        RetryConfigBuilder::new()
    }
    
    pub fn mode(&self) -> &RetryMode {
        &self.mode
    }
    
    pub fn max_attempts(&self) -> &u32 {
        &self.max_attempts
    }
    
    pub fn initial_delay(&self) -> &Duration {
        &self.initial_delay
    }
    
    pub fn max_delay(&self) -> &Duration {
        &self.max_delay
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
            mode: RetryMode::Off,
            max_attempts: 1,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetryConfigBuilder {
    mode: RetryMode,
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
}

impl RetryConfigBuilder {
    pub fn new() -> Self {
        Self {
            mode: RetryMode::Standard,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }

    pub fn mode(mut self, mode: RetryMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    pub fn build(self) -> RetryConfig {
        RetryConfig {
            mode: self.mode,
            max_attempts: self.max_attempts,
            initial_delay: self.initial_delay,
            max_delay: self.max_delay,
            backoff_multiplier: self.backoff_multiplier,
        }
    }
}

impl Default for RetryConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}