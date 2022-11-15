use chrono::{DateTime, Utc};
use anyhow::anyhow;
#[path = "./authclient.rs"] mod authclient;
use authclient::make_auth_request;

// Rust
// struct + impl = "class"
#[derive(Copy, Clone)]
pub struct CircuitBreaker<'a> {
    circuit_breaker_state: &'a str,
    success_count: i64,
    fail_count: i64,
    timeout_reset: i64,
    timeout_open_state: i64,
    trigger_half_state: i64,
    trigger_open_state: i64,
    trigger_closed_state: i64,
    timestamp: DateTime<Utc>,
    permitted_requests_in_state_half: i64,
    request_count: i64,
    hostname:  &'a str,
    port: i32,
}
impl <'a> CircuitBreaker<'a>  {
    pub fn new(timeout_reset: i64, timeout_open_state: i64, trigger_half_state: i64, trigger_open_state: i64,
               permitted_requests_in_state_half: i64, trigger_closed_state: i64, hostname: &'a str, port: i32) -> Self {

        return Self { circuit_breaker_state: "CLOSED", success_count: 0, fail_count: 0 , timeout_reset, timeout_open_state,
            trigger_half_state, trigger_closed_state, timestamp: Utc::now(), trigger_open_state, permitted_requests_in_state_half,
            request_count: 0, hostname, port};
    }

    fn check_reset(&mut self, time_diff: i64) {
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

    pub async fn circuit_breaker_post_request(&mut self, addr_with_params: String) -> Result<std::string::String, anyhow::Error> {

        let current_timestamp = Utc::now();
        let time_diff = current_timestamp.signed_duration_since(self.timestamp).num_seconds();
        println!("timeDiff is {}", time_diff);

        self.check_reset(time_diff);

        if self.circuit_breaker_state == "OPEN" {

            if time_diff >= self.timeout_open_state {
                // Wenn timeout abgelaufen setze den Circuit Breaker wieder auf HALF
                println!("Circuit Breaker: Wechsel Circuit Breaker Status von OPEN auf HALF");
                self.circuit_breaker_state = "HALF";

            } else {
                println!("Circuit Breaker: immer noch auf Zustand OPEN");
                println!("Circuit Breaker ist {} noch offen", (self.timeout_open_state - time_diff));
                return Err(anyhow!("Request fehlgeschlagen: Circuit Breaker ist im Zustand offen und keine Requests sind zum Service Benutzerverwaltung erlaubt"))
            }

        }

        if self.circuit_breaker_state == "HALF" && self.request_count > self.permitted_requests_in_state_half {
            println!("Request fehlgeschlagen: Circuit Breaker ist auf Zustand HALF aber der erlaubte RequestCount ist erreicht");
            return Err(anyhow!("Request fehlgeschlagen: Circuit Breaker ist auf Zustand HALF aber der erlaubte RequestCount ist erreicht"))
        }

        if self.circuit_breaker_state == "HALF" && (self.success_count - self.fail_count > self.trigger_closed_state) {
            self.check_reset(time_diff);
            println!("Circuit Breaker: Wechsel Circuit Breaker Status von HALF auf CLOSED");
            self.circuit_breaker_state = "CLOSED";
        }

        println!("Circuit Breaker: Führe HTTP Request im Circuit Breaker durch");
        if self.circuit_breaker_state == "HALF" {
            self.request_count += 1;
        }

        let url = format!("{}:{}{}", self.hostname, self.port, addr_with_params);

        match authclient::make_auth_request(url).await {
            Ok((http_code, result)) => {
                self.success_count += 1;
                println!("Circuit Breaker: Request war erfolgreich. Success Count ist jetzt bei {}", self.success_count);
                if http_code == "200" {
                    return Ok(result)
                } else {

                    self.fail_count += 1;
                    if self.circuit_breaker_state == "CLOSED" && self.success_count - self.fail_count < self.trigger_half_state {
                        println!("Circuit Breaker: Wechsel Circuit Breaker Status von CLOSED auf HALF");
                        self.circuit_breaker_state = "HALF";
                        self.timestamp = Utc::now();
                    }

                    if self.circuit_breaker_state == "HALF" && (self.success_count - self.fail_count < self.trigger_open_state) {
                        println!("Circuit Breaker: Wechsel Circuit Breaker Status von HALF auf OPEN");
                        self.circuit_breaker_state = "OPEN";
                        self.timestamp = Utc::now();
                    }

                    println!("Circuit Breaker: Request ist fehlgeschlagen. Fail Count ist jetzt bei {}", self.fail_count);
                    return Err(anyhow!("CircuitBreaker: Request failed, return code was {}", http_code))

                }


            },
            Err(err) => {
                println!("{:?}", err);
                return Err(anyhow!("CircuitBreaker: Request failed, return code was 500"))
            },
        }

    }



}


