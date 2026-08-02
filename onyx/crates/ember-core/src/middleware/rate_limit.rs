use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use ember_http::{request::Request, response::Response, status::StatusCode};

use super::{Middleware, Next};

struct Bucket {
    tokens: usize,
    last_refill: Instant,
}

pub struct RateLimiter {
    capacity: usize,
    refill_interval: Duration,
    buckets: Mutex<HashMap<String, Bucket>>,
}

impl RateLimiter {
    pub fn new(capacity: usize, refill_interval: Duration) -> Self {
        Self {
            capacity,
            refill_interval,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    fn allow(&self, client: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();

        let bucket = buckets.entry(client.to_string()).or_insert(Bucket {
            tokens: self.capacity,
            last_refill: Instant::now(),
        });

        let elapsed = bucket.last_refill.elapsed();

        if elapsed >= self.refill_interval {
            bucket.tokens = self.capacity;
            bucket.last_refill = Instant::now();
        }

        if bucket.tokens == 0 {
            return false;
        }

        bucket.tokens -= 1;

        true
    }
}

impl Middleware for RateLimiter {
    fn handle(&self, request: Request, next: &Next) -> Response {
        // Tier 2: temporary client identifier
        let client = "global";

        if !self.allow(client) {
            return Response::new(StatusCode::TooManyRequests)
                .header("Content-Type", "text/plain")
                .body("429 Too Many Requests");
        }

        next.run(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_initial_requests() {
        let limiter = RateLimiter::new(2, Duration::from_secs(60));

        assert!(limiter.allow("client"));
        assert!(limiter.allow("client"));
    }

    #[test]
    fn blocks_when_bucket_empty() {
        let limiter = RateLimiter::new(1, Duration::from_secs(60));

        assert!(limiter.allow("client"));
        assert!(!limiter.allow("client"));
    }
}
