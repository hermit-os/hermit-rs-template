#[cfg(target_os = "hermit")]
use hermit as _;

use axum::Router;
use axum::routing::get;
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    eprintln!("Reached main");

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;

    let app = Router::new().route("/", get(root));

    eprintln!("Listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!\n"
}
