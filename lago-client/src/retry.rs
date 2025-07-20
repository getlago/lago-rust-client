use std::time::Duration;

/// Defines the retry behavior for failed requests
///
/// This enum controls how the client handles retry attempts when requests fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryMode {
    Off,
    Standard,
    Adaptive,
}

impl Default for RetryMode {
    /// Returns the default retry mode (Standard)
    ///
    /// # Returns
    /// `RetryMode::Standard` as the default retry behavior
    fn default() -> Self {
        RetryMode::Standard
    }
}

/// Configuration settings for retry behavior
///
/// This struct contains all the parameters needed to configure how the client
/// handles retry attempts, including timing, limits, and backoff strategies.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub(crate) mode: RetryMode,
    pub(crate) max_attempts: u32,
    pub(crate) initial_delay: Duration,
    pub(crate) max_delay: Duration,
    pub(crate) backoff_multiplier: f64,
}

impl RetryConfig {
    /// Creates a new retry configuration with default settings
    ///
    /// # Returns
    /// A new `RetryConfig` instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new retry configuration builder
    ///
    /// # Returns
    /// A new `RetryConfigBuilder` instance for constructing custom retry settings
    pub fn builder() -> RetryConfigBuilder {
        RetryConfigBuilder::new()
    }

    /// Gets the retry mode
    ///
    /// # Returns
    /// A reference to the current retry mode
    pub fn mode(&self) -> &RetryMode {
        &self.mode
    }

    /// Gets the maximum number of retry attempts
    ///
    /// # Returns
    /// A reference to the maximum attempts setting
    pub fn max_attempts(&self) -> &u32 {
        &self.max_attempts
    }

    /// Gets the initial delay for retry attempts
    ///
    /// # Returns
    /// A reference to the initial delay duration
    pub fn initial_delay(&self) -> &Duration {
        &self.initial_delay
    }

    /// Gets the maximum delay between retry attempts
    ///
    /// # Returns
    /// A reference to the maximum delay duration
    pub fn max_delay(&self) -> &Duration {
        &self.max_delay
    }

    /// Calculates the delay duration for a specific retry attempt
    ///
    /// This method implements exponential backoff with a configurable multiplier
    /// and enforces the maximum delay limit.
    ///
    /// # Arguments
    /// * `attempt` - The attempt number (0-based)
    ///
    /// # Returns
    /// The duration to wait before the next retry attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if self.mode == RetryMode::Off {
            return Duration::from_secs(0);
        }

        let delay_secs =
            self.initial_delay.as_secs_f64() * self.backoff_multiplier.powi(attempt as i32);

        let delay = Duration::from_secs_f64(delay_secs);

        if delay > self.max_delay {
            self.max_delay
        } else {
            delay
        }
    }
}

impl Default for RetryConfig {
    /// Creates a default retry configuration
    ///
    /// By default, retries are disabled (mode is Off) with conservative settings
    /// that can be overridden when building a custom configuration.
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

/// Builder for creating customized retry configuration instances
///
/// This builder allows you to configure various aspects of retry behavior
/// such as retry mode, maximum attempts, delays, and backoff multipliers.
#[derive(Debug, Clone)]
pub struct RetryConfigBuilder {
    mode: RetryMode,
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
}

impl RetryConfigBuilder {
    /// Creates a new retry configuration builder with sensible defaults
    ///
    /// # Returns
    /// A new `RetryConfigBuilder` instance with default retry settings
    pub fn new() -> Self {
        Self {
            mode: RetryMode::Standard,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }

    /// Sets the retry mode
    ///
    /// # Arguments
    /// * `mode` - The retry mode to use
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn mode(mut self, mode: RetryMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the maximum number of retry attempts
    ///
    /// # Arguments
    /// * `max_attempts` - The maximum number of retry attempts
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Sets the initial delay for retry attempts
    ///
    /// # Arguments
    /// * `delay` - The initial delay duration
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Sets the maximum delay between retry attempts
    ///
    /// # Arguments
    /// * `delay` - The maximum delay duration
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Sets the backoff multiplier for exponential backoff
    ///
    /// # Arguments
    /// * `multiplier` - The multiplier to apply to delays between attempts
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Builds the final retry configuration instance
    ///
    /// # Returns
    /// A new `RetryConfig` instance with the specified settings
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
    /// Creates a default retry configuration builder
    ///
    /// This is equivalent to calling `RetryConfigBuilder::new()`.
    fn default() -> Self {
        Self::new()
    }
}
