use backend::server::_server;

#[actix_web::main]
async fn main() {
    _server().await.unwrap()
}