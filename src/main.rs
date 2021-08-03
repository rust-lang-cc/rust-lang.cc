

use trillium::Conn;
// use trillium::{Conn, Handler};
use trillium_logger::Logger;
use trillium_router::Router;
// use trillium_router::{Router, RouterConnExt};
use trillium_askama::{AskamaConnExt, Template};
// use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Template)]
#[template(path = "tpl/index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn hello(conn: Conn) -> Conn {
    conn.render(IndexTemplate { name: "world" })
}


#[derive(Template)]
#[template(path = "tpl/who_and_how.html")]
struct Index2Template<'a> {
    name: &'a str,
}

async fn hello2(conn: Conn) -> Conn {
    conn.render(Index2Template { name: "world" })
}


fn main() {
    env_logger::init();
    trillium_smol::config()
        .with_port(8080)
        .with_host("0.0.0.0")
        // .with_host("::")
        .run((
            Logger::new(),
            Router::new()
                .get("/", hello)
                .get("/who_and_how.html", hello2),
        ));
}
