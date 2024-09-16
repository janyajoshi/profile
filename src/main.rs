use actix_web::{get, web::ServiceConfig, HttpRequest};
use shuttle_actix_web::ShuttleActixWeb;

mod utils {
    pub mod ascii_utils;
    pub mod profile;
}

#[get("/")]
async fn contact(http_request: HttpRequest) -> String {
    utils::profile::get_profile(http_request, false)
}

#[get("/r")]
async fn detail(http_request: HttpRequest) -> String {
    utils::profile::get_profile(http_request, true)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(contact);
        cfg.service(detail);
    };

    Ok(config.into())
}
