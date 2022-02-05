use once_cell::sync::Lazy;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::Response;
use rocket::{http::Status, Request};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

static RATE_LIMIT_SESSION_NAME: &str = "rate_limit_session";
static RATE_LIMIT_HEADER_LIMIT_NAME: &str = "X-RateLimit-Limit";
static RATE_LIMIT_HEADER_REMAINING_NAME: &str = "X-RateLimit-Remaining";

static RATE_LIMITER_ENTRIES: Lazy<Mutex<HashMap<String, RateLimiterEntry>>> =
    Lazy::new(|| Mutex::default());

#[derive(Debug, Clone)]
struct RateLimiterEntry {
    ip: Option<IpAddr>,
    remaining: u64,
    last_access: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    limit: u64,
    duration: Duration,
}

impl RateLimiter {
    pub fn new(limit: u64, duration: Duration) -> Self {
        Self { limit, duration }
    }
}

#[rocket::async_trait]
impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limiter",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // If is an error we ignore it
        if response.status().code >= 400 {
            return;
        }

        let key = get_request_key(request, response);
        let ip = request.client_ip();
        let mut rate_limiter_entries = RATE_LIMITER_ENTRIES.lock().unwrap();
        let entry_option = rate_limiter_entries.get(&key).cloned();
        let is_first_access = entry_option.is_none();
        let mut entry = entry_option.unwrap_or_else(|| RateLimiterEntry {
            ip,
            remaining: self.limit,
            last_access: Instant::now(),
        });

        let limit = self.limit;
        let duration = self.duration;
        let elapsed = entry.last_access.elapsed();

        // Reset the counter
        if elapsed > duration {
            entry.remaining = limit;
            entry.last_access = Instant::now();
        } else {
            if !is_first_access && entry.remaining > 0 {
                entry.remaining -= 1;
            }
        }

        // Sets the rate limiter headers
        set_rate_limiter_response(response, &entry, &self);

        // Update the entry
        rate_limiter_entries.insert(key, entry);
    }
}

fn set_rate_limiter_response(
    response: &mut Response,
    entry: &RateLimiterEntry,
    rate_limiter: &RateLimiter,
) {
    response.set_header(Header::new(
        RATE_LIMIT_HEADER_LIMIT_NAME.to_string(),
        rate_limiter.limit.to_string(),
    ));
    response.set_header(Header::new(
        RATE_LIMIT_HEADER_REMAINING_NAME.to_string(),
        entry.remaining.to_string(),
    ));

    if entry.remaining == 0 {
        response.set_status(Status::TooManyRequests);
        let _ = response.body_mut().take();
    }
}

fn get_request_key(request: &Request<'_>, response: &mut Response<'_>) -> String {
    if let Some(ip) = request.client_ip() {
        return ip.to_string();
    }

    if let Some(cookie) = request.cookies().get_private(RATE_LIMIT_SESSION_NAME) {
        cookie.value().to_string()
    } else {
        let key = uuid::Uuid::new_v4().to_string();
        response.set_header(Header::new(
            "Set-Cookie",
            format!("{}={}", RATE_LIMIT_SESSION_NAME, key),
        ));
        key
    }
}
