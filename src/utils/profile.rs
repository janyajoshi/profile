use actix_web::http::header::{USER_AGENT};
use actix_web::HttpRequest;
use crate::utils::ascii_utils::{contact, detail};

pub fn get_profile(http_request: HttpRequest, get_detail: bool) -> String {
    let user_agent = http_request.headers().get(USER_AGENT);
    let agent = match user_agent {
        Some(agent) => agent.to_str().unwrap_or_else(|_| "Invalid user agent header"),
        None => "User Agent Not Found",
    };

    if !agent.contains("curl") {
        String::from(format!("run \"curl {}\"", http_request.full_url()))
    } else if get_detail {
        String::from(detail())
    } else { String::from(contact()) }
}