use trillium::{conn_unwrap, Conn};
use trillium_router::{Router, RouterConnExt};
use trillium_rustls::RustlsConnector;
use trillium_smol::TcpConnector;

type Proxy = trillium_proxy::Proxy<RustlsConnector<TcpConnector>>;

pub async fn hello_world(conn: Conn) -> Conn {
    conn.ok("hello world!")
}

pub async fn hello_name(conn: Conn) -> Conn {
    let name = conn_unwrap!(conn, conn.param("name"));
    let body = format!("hello, {}!", name);
    conn.ok(body)
}

pub async fn not_found(conn: Conn) -> Conn {
    let body = format!("Uh oh, I don't have a route for {}", conn.path());
    conn.with_body(body).with_status(404)
}

pub fn router() -> Router {
    Router::new()
        .get("/trillium-template/", hello_world)
        .get("/trillium-template/hello/:name", hello_name)
        .get("/httpbin/*", Proxy::new("https://httpbin.org"))
        .get("/trillium/*", Proxy::new("https://trillium.rs"))
}
