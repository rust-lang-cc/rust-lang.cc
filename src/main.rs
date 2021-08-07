use trillium::Conn;
// use trillium_logger::Logger;
// use trillium_router::Router;
use trillium_askama::{AskamaConnExt, Template};
// use trillium_static_compiled::{include_dir, StaticCompiledHandler};
// use std::net::{Ipv4Addr, Ipv6Addr};

mod application;
mod routes;
mod tests;
use application::application;

#[derive(Template)]
#[template(path = "tpl/index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn hello(conn: Conn) -> Conn {
    conn.render(IndexTemplate { name: "world" })
}


fn main() {
    pretty_env_logger::init();
	
    #[cfg(unix)]  // comment this line on local developing.
    trillium_smol::config()
        .with_port(8080)
        // .with_host("0.0.0.0") // this line for ipv4, next line for both for ipv4 and ipv6 on fly.io
        .with_host("::")
		.run(application());
		
        /* .run((
            StaticCompiledHandler::new(include_dir!("./templates/tpl")).with_index_file("index.html"),
            // Router::new()
                // .get("/", hello),
        )); */
}
