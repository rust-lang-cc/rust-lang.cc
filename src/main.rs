use poem::{listener::TcpListener, route, service::Files, Server};

#[tokio::main]
async fn main() {
    let app = route().nest(
        "/",
<<<<<<< HEAD
        Files::new("/app/front-end/").show_files_listing().index_file("index.html"),
=======
        Files::new("/").show_files_listing(),
>>>>>>> bb02a7dc1795dc2e659692e6bd5fdf7cdb15d25e
    );
    let server = Server::new(TcpListener::bind("0.0.0.0:8080"))
        .await
        .unwrap();
    server.run(app).await.unwrap();
}
