use std::time::{Duration, Instant};

pub const TEST_TIMEOUT: Duration = Duration::from_secs(5);

pub fn run_with_timeout<F, T>(test_fn: F) -> Result<T, String>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::spawn(move || {
        let start = Instant::now();
        let result = test_fn();
        if start.elapsed() > TEST_TIMEOUT {
            panic!("Test timed out");
        }
        result
    });

    handle
        .join()
        .map_err(|_| "Test panicked or timed out".to_string())
}