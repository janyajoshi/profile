use actix_web::{get, web::ServiceConfig, HttpRequest, Responder};
use shuttle_actix_web::ShuttleActixWeb;
use actix_files::Files;

mod utils {
    pub mod ascii_utils;
    pub mod profile;
    pub mod info;
}

#[get("/")]
async fn contact(http_request: HttpRequest) -> impl Responder {
    utils::profile::get_profile(http_request, false)
}

#[get("/r")]
async fn detail(http_request: HttpRequest) -> impl Responder {
    utils::profile::get_profile(http_request, true)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(contact);
        cfg.service(detail);
        cfg.service(Files::new("/assets", "assets"));
    };

    Ok(config.into())
}
