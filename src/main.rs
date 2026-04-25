use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::{self, Duration, Instant},
};

struct RateLimiter {
    duration: Duration,
    requests: Arc<Mutex<VecDeque<Instant>>>,
    max_request: usize,
}

impl RateLimiter {
    fn new(max_reqs: usize, duration: Duration) -> Self {
        Self {
            duration,
            requests: Arc::new(Mutex::new(VecDeque::new())),
            max_request: max_reqs,
        }
    }
    fn is_available(&mut self) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.lock().unwrap();

        while let Some(&oldest) = requests.front() {
            if now.duration_since(oldest) >= self.duration {
                requests.pop_front();
            } else {
                break;
            }
        }

        if requests.len() < self.max_request {
            requests.push_back(now);
            true
        } else {
            false
        }
    }
}

fn main() {
    // usage pattern 2:
    // our rate limiter is not good for this pattern. It allows 20 requests in almost 2 seconds!
    println!("Second scenario");
    let started = time::Instant::now();
    let mut rl = RateLimiter::new(10, Duration::from_secs(10));
    thread::sleep(Duration::from_secs(9));
    let mut r = 0;
    for _ in 0..10 {
        r += 1;
        println!(
            "  {}  is service available at {:?}? {}",
            r,
            time::Instant::now() - started,
            rl.is_available()
        );
    }
    thread::sleep(Duration::from_secs(2));
    for _ in 0..10 {
        r += 1;
        println!(
            "  {}  is service available at {:?}? {}",
            r,
            time::Instant::now() - started,
            rl.is_available()
        );
        thread::sleep(Duration::from_secs(2));
    }
    // end of pattern 2
}
