use poem::{listener::TcpListener, route, service::Files, Server};

#[tokio::main]
async fn main() {
    let app = route().nest(
        "/",
        Files::new("./front-end/").show_files_listing().index_file("index.html"),
    );
    let server = Server::new(TcpListener::bind("0.0.0.0:8080"))
        .await
        .unwrap();
    server.run(app).await.unwrap();
}
