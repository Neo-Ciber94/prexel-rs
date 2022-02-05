use std::{collections::HashMap, marker::PhantomData, net::IpAddr, time::Duration};

use once_cell::sync::Lazy;
use rocket::{request::{FromRequest, Outcome}, State, Request};

#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    pub limit: u64,
    pub cold_down: Duration,
}

pub trait RateLimiterConfigProvider {
    fn get_config(&self) -> RateLimiterConfig;
}


#[derive(Default)]
pub struct RateLimit<P: RateLimiterConfigProvider> {
    _marker: PhantomData<P>,
}

impl<P: RateLimiterConfigProvider> RateLimit<P> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

struct RateLimiterInfo {
    ip: IpAddr,
    remaining: u64,
    last_request: std::time::Instant,
}

static RATE_LIMITER_MAP: Lazy<HashMap<IpAddr, RateLimiterInfo>> = Lazy::new(|| HashMap::new());

#[rocket::async_trait]
impl<'r, P> FromRequest<'r> for RateLimit<P> where P: RateLimiterConfigProvider + Send + Sync + 'static {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // We can't apply rate-limit if there is no ip.
        if let None = request.client_ip() {
            return Outcome::Forward(());
        }

        let ip = request.client_ip().unwrap();
        let provider = request.rocket().state::<P>().unwrap();
        let config = provider.get_config();
        let headers = request.headers();

        todo!()
    }
}
