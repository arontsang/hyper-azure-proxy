use std::{env, net::Ipv4Addr, io::Error};
use axum::handler::HandlerWithoutStateExt;

#[compio::main]
async fn main() -> Result<(), Error>{
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let listener = compio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await.unwrap();
    cyper_axum::serve(listener, handler.into_make_service())
        .await
}

async fn handler() -> &'static str {
    "Hello, World!"
}