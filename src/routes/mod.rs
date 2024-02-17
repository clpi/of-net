pub mod admin;
pub mod group;
pub mod user;

use http::StatusCode;
use ntex::web::{self, get, HttpRequest, Responder};
use ntex_io::IoStatusUpdate;

// #[get("/user")]
pub async fn users(r: HttpRequest) -> web::HttpResponse {
    let mut resp = web::HttpResponse::Ok().force_close().finish();
    resp.head_mut()
        .set_connection_type(ntex::http::ConnectionType::Close);
    resp
}

// #[get("/")]
pub async fn index(r: HttpRequest) -> web::HttpResponse {
    let mut resp = web::HttpResponse::Ok().force_close().finish();
    resp.head_mut()
        .set_connection_type(ntex::http::ConnectionType::Close);
    resp
}
// #[get("/about")]
pub async fn about(r: HttpRequest) -> impl web::Responder {
    match r.method() {
        &http::Method::GET => web::HttpResponse::Ok().force_close().finish(),
        _ => web::HttpResponse::Ok().force_close().finish(),
    }
}
