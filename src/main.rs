use trillium::{Conn, Handler};
use trillium_logger::Logger;
use trillium_router::{Router, RouterConnExt};
// use trillium_askama::{AskamaConnExt, Template};

async fn hello(conn: Conn) -> Conn {
    conn.ok("hello")
}

fn main() {
    env_logger::init();
    trillium_smol::config()
        .with_port(8080)
        .with_host("::")
        .run((
            Logger::new(),
            Router::new()
                .get("/", hello),
        ));
}
