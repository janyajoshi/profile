use actix_web::{get, web::ServiceConfig, HttpRequest};
use shuttle_actix_web::ShuttleActixWeb;

mod utils {
    pub mod ascii_utils;
    pub mod profile;
}

#[get("/")]
async fn hello_world(http_request: HttpRequest) -> String {
    utils::profile::get_profile(http_request)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
