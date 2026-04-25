# Rate Limiter

A simple rate limiter implementation in Rust for learning how request throttling works.

**Not suitable for production use** 

This rate limiter uses a sliding window approach to track and limit requests over a specified time duration. It's useful for controlling API usage, rate-limiting connections, or throttling any resource access pattern.

## How It Works

The `RateLimiter` maintains a time-window based queue of request timestamps. When a request comes in:

1. **Remove expired requests**: Any requests older than the specified duration are cleared from the queue
2. **Check capacity**: If the number of remaining requests is below the limit, the request is allowed
3. **Track the request**: Allowed requests are added to the queue with their timestamp

## Usage

```rust
use std::time::Duration;

fn main() {
    // Create a rate limiter: 10 requests per 10 seconds
    let mut rate_limiter = RateLimiter::new(10, Duration::from_secs(10));

    // Check if a request is allowed
    if rate_limiter.is_available() {
        println!("Request allowed ✓");
    } else {
        println!("Rate limit exceeded, try again later");
    }
}
```

## License

Feel free to use and modify this rate limiter for your projects!
