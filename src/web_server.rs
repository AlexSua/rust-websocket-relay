mod controller;
mod models;
mod websocket;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix::Addr;
use actix_web::{web, App, HttpServer};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::cli::{Cli, Commands};
use websocket::{MyData, MyWs};

#[actix_web::main]
pub async fn web_server_start(cli: &Cli) -> std::io::Result<()> {
    let channels: Arc<RwLock<HashMap<String, Addr<MyWs>>>> = Arc::new(RwLock::new(HashMap::new()));

    let mut openssl = false;
    let mut uri_cli_param = match &cli.scope {
        Some(uri) => uri.to_string(),
        None => "".to_owned(),
    };

    let uri_cli_param_clone = uri_cli_param.clone();
    let mut http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(MyData {
                channels: channels.clone(),
            }))
            .service(
                web::scope(uri_cli_param_clone.as_str()).service(controller::websocket_handler),
            )
    });
    
    match &cli.command {
        Some(Commands::Ssl {
            certificate_path,
            privkey_path,
        }) => {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            let ip_port_string = cli.ip.to_string() + ":" + cli.port.to_string().as_str();
            openssl = true;

            builder
                .set_private_key_file(privkey_path, SslFiletype::PEM)
                .unwrap();
            builder
                .set_certificate_chain_file(certificate_path)
                .unwrap();

            http_server = http_server.bind_openssl(ip_port_string, builder)?;
        }
        None => {
            http_server = http_server.bind((cli.ip, cli.port))?;
        }
    }
    let server = http_server.run();

    let mut websocket_extension = "ws://";
    if openssl {
        websocket_extension = "wss://";
    }

    if uri_cli_param.starts_with("/") && uri_cli_param != "/" {
        uri_cli_param.remove(0);
    }

    println!(
        "- Websocket url: {}{}:{}/{}<id>",
        websocket_extension, cli.ip, cli.port, uri_cli_param
    );

    server.await
}
