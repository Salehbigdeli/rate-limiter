use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{self, Duration},
};

struct RateLimiterState {
    duration: Duration,
    max_requests: u64,
    requests_count: Arc<AtomicU64>,
}

impl RateLimiterState {
    fn start(&self) {
        let millis = self.duration.as_millis() as u64 / self.max_requests;
        let duration = Duration::from_millis(millis);
        let requests_count = Arc::clone(&self.requests_count);
        thread::spawn(move || {
            loop {
                thread::sleep(duration);
                let _ = requests_count
                    .fetch_update(Ordering::SeqCst, Ordering::Relaxed, |x| {
                        if x > 0 { Some(x - 1) } else { None }
                    });
            }
        });
    }
}
struct RateLimiter {
    state: RateLimiterState,
}

impl RateLimiter {
    fn new(max_reqs: u64, duration: Duration) -> Self {
        let state = RateLimiterState {
            duration,
            max_requests: max_reqs,
            requests_count: Arc::new(AtomicU64::new(0)),
        };
        state.start();
        Self { state }
    }
    fn is_available(&mut self) -> bool {
        if self.state.requests_count.load(Ordering::Relaxed) < self.state.max_requests {
            self.state.requests_count.fetch_add(1, Ordering::Relaxed);
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
    for _ in 0..10 {
        println!(
            "    is service available at {:?}? {}",
            time::Instant::now() - started,
            rl.is_available()
        );
    }
    thread::sleep(Duration::from_secs(2));
    for _ in 0..10 {
        println!(
            "    is service available at {:?}? {}",
            time::Instant::now() - started,
            rl.is_available()
        );
    }
    // end of pattern 2
}
