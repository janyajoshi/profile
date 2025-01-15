use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use actix_files::Files;

mod utils {
    pub mod ascii_utils;
    pub mod profile;
    pub mod info;
    pub mod ascii_to_html;
}

#[get("/")]
async fn contact(http_request: HttpRequest) -> impl Responder {
    utils::profile::get_profile(http_request, false)
}

#[get("/r")]
async fn detail(http_request: HttpRequest) -> impl Responder {
    utils::profile::get_profile(http_request, true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(contact)
            .service(detail)
            .service(Files::new("/assets", "assets"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
