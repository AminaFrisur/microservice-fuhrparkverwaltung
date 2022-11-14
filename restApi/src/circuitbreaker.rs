use chrono::{DateTime, Duration, Utc};

// Rust
// struct + impl = "class"
pub struct CircuitBreaker {
    circuit_breaker_state: String,
    success_count: i32,
    fail_count: i32,
    timeout_reset: i32,
    timeout_open_state: i32,
    trigger_half_state: i32,
    trigger_open_state: i32,
    trigger_closed_state: i32,
    timestamp: DateTime<Utc>,
    permitted_requests_in_state_half: i32,
    request_count: i32,
    hostname: String,
    port: i32,
}
impl CircuitBreaker {

    pub fn new(timeout_reset: i32, timeout_open_state: i32, trigger_half_state: i32, trigger_open_state: i32,
               permitted_requests_in_state_half: i32, trigger_closed_state: i32, hostname: String, port: i32) -> Self {

        return Self { circuit_breaker_state: format!("CLOSED"), success_count: 0, fail_count: 0 , timeout_reset, timeout_open_state,
            trigger_half_state, trigger_closed_state, timestamp: Utc::now(), trigger_open_state, permitted_requests_in_state_half,
            request_count: 0, hostname, port};
    }

    fn check_reset(&mut self, time_diff: i32) {
        println!("{}", time_diff);
        println!("{}", time_diff - self.timeout_reset);
        println!("Circuit Breaker: Prüfe ob Circuit Breaker Status zurückgesetzt werden soll");
        if time_diff > self.timeout_reset &&
            (self.circuit_breaker_state == format!("CLOSED") || self.circuit_breaker_state == format!("HALF")) {
            println!("Circuit Breaker: Kompletter Status wird zurückgesetzt!");
            self.fail_count = 0;
            self.success_count = 0;
            self.timestamp = Utc::now();
            self.request_count = 0;
        }
    }

    pub fn circuit_breaker_post_request(&self, path: &str, params: Box<[&str]>) {


    }



}


