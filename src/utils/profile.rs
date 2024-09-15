use actix_web::http::header::{HOST, USER_AGENT};
use actix_web::HttpRequest;

pub fn get_profile(http_request: HttpRequest) -> String {
    let user_agent = http_request.headers().get(USER_AGENT);
    let agent = match user_agent {
        Some(agent) => agent.to_str().unwrap_or_else(|_| "Invalid user agent header"),
        None => "User Agent Not Found",
    };

    let user_host = http_request.headers().get(HOST);
    let host = match user_host {
        Some(agent) => agent.to_str().unwrap_or_else(|_| "Invalid user host"),
        None => "host Not Found",
    };

    if !agent.contains("curl") {
        String::from( format!("run \"curl https://{}\"", host))
    } else {
        String::from(crate::utils::ascii_utils::runner())
    }
}