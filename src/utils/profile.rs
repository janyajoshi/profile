use std::fs;
use actix_web::http::header::{USER_AGENT};
use actix_web::{HttpRequest, HttpResponse, Responder};
use async_stream::stream;
use crate::utils::ascii_utils::{contact, detail};
use actix_web::web::Bytes;
use tokio::time::{sleep, Duration};

pub fn get_profile(http_request: HttpRequest, get_detail: bool) -> impl Responder {
    let user_agent = http_request.headers().get(USER_AGENT);
    let agent = match user_agent {
        Some(agent) => agent.to_str().unwrap_or_else(|_| "Invalid user agent header"),
        None => "User Agent Not Found",
    };

    if !agent.contains("curl") {
        let html = fs::read_to_string("assets/profile.html").unwrap();
        let url = http_request.full_url().to_string().replace("http:", "https:");
        let html_content = html.replace("{url}", &url);
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html_content)
    } else {
        let res = if get_detail { String::from(detail()) } else { String::from(contact()) };
        let stream = stream! {
            for line in res.lines() {
                yield Ok::<Bytes, std::io::Error>(Bytes::from(format!("{}\n", line).to_string()));
                sleep(Duration::from_millis(100)).await;
            }
        };
        HttpResponse::Ok()
            .content_type("text/plain")
            .streaming(stream)
    }
}