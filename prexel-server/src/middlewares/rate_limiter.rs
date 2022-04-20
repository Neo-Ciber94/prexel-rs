use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use uuid::Uuid;

pub const RATE_LIMIT_ID : &str = "X-Ratelimit-Id";

pub fn get_rate_limit_identifier(req: &ServiceRequest) -> Option<String> {
    fn internal_get_identifier(req: &ServiceRequest) -> Option<String> {
        if let Some(remote_addr) = req.connection_info().remote_addr() {
            return Some(remote_addr.to_string());
        }

        // FIXME: Peer address can be trick with a proxy
        if let Some(peer_addr) = req.peer_addr() {
            return Some(peer_addr.to_string());
        }

        if let Some(cookie) = req.cookie(RATE_LIMIT_ID) {
            if cookie.http_only().unwrap_or(true) {
                return Some(cookie.value().to_string());
            }
        }

        None
    }

    internal_get_identifier(req).map(|id| {
        base64::encode(id).trim_end_matches('=').to_string()
    })
}

pub fn generate_rate_limit_id() -> String {
    let uuid = Uuid::new_v4();
    base64::encode(uuid.as_bytes()).trim_end_matches('=').to_string()
}