use poem::{listener::TcpListener, route, service::Files, Server};

#[tokio::main]
async fn main() {
    let app = route().nest(
        "/",
        Files::new("/").show_files_listing(),
    );
    let server = Server::new(TcpListener::bind("0.0.0.0:8080"))
        .await
        .unwrap();
    server.run(app).await.unwrap();
}
