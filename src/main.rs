use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"] // Specify the folder containing your assets
struct Asset;

mod utils {
    pub mod ascii_utils;
    pub mod profile;
    pub mod info;
    pub mod ascii_to_html;
}

#[get("/")]
async fn contact(http_request: HttpRequest) -> impl Responder {
    println!("/, {:?}", http_request.headers().get("user-agent")
        .map_or_else(|| "", |value| value.to_str().unwrap_or("")));
    utils::profile::get_profile(http_request, false)
}

#[get("/r")]
async fn detail(http_request: HttpRequest) -> impl Responder {
    println!("/r, {:?}", http_request.headers().get("user-agent")
        .map_or_else(|| "", |value| value.to_str().unwrap_or("")));
    utils::profile::get_profile(http_request, true)
}

async fn serve_asset(req: HttpRequest) -> impl Responder {
    let path: String = req.match_info().query("filename").to_string();
    match Asset::get(&path) {
        Some(file) => {
            actix_web::HttpResponse::Ok()
                .content_type("application/octet-stream") // auto infer MIME type
                .body(file.data)
        }
        None => {
            actix_web::HttpResponse::NotFound().body("File not found")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("started server - best of luck");
    HttpServer::new(|| {
        App::new()
            .service(contact)
            .service(detail)
            .route("/assets/{filename:.*}", web::get().to(serve_asset))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
