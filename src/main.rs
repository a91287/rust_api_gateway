#![deny(warnings)]
#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use log::{error, info};
use hyper::service::service_fn;
use tokio::net::{TcpListener, TcpStream};
mod settings;
use settings::Settings;
mod logging;

mod request_handler;
mod request_filter;
use request_handler::handle_request;
use hyper::server::conn::Http;


//lazy loading the configuration only once at the startup.
lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("unable to create settings");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /*
    First step is to setup the logging configuration from the
    settings on the application configuration file. 
    */
    //BEGIN logging settup
    let lpath = SETTINGS.clone().get_logging().get_log_path();
    let llevel = SETTINGS.clone().get_logging().get_log_level();
    let lpstdout = SETTINGS.clone().get_logging().get_std_out();
    let log_file_size_in_bytes = SETTINGS.clone().get_logging().get_log_file_size_in_bytes();

    let logger_cfg = vec![
        ("log_path", lpath.as_str()),
        ("log_level", llevel.as_str()),
        ("std_out", lpstdout.as_str()),
        ("log_file_size_in_bytes", log_file_size_in_bytes.as_str()),
        
    ];

    let _=logging::logger::setup_logger(logger_cfg);
    //END logger setup

    info!("Application Started !");

    /* 
    Setup a listening address and start listening for new connections
    from http clients
    */
    //BEGIN setup the connection listening address
    let listening_addr= SETTINGS.clone().get_listening_address();
    let addr: SocketAddr = listening_addr.parse().expect("Error parsing the listening address");
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    //END setup the connection listening address

    loop {
        // Asynchronously wait for an inbound TcpStream
        let (tcp, _) = listener.accept().await?;
        

        // Spawn a new task to process each connection
        tokio::task::spawn(async move {
            /*
            We're using a simple HTTP server here; for HTTPS, you would wrap this stream
            with a TLS acceptor.
            */
            //TODO: setup SSL
            if let Err(e) = serve_connection(tcp).await {
                error!("Error while serving connection: {:?}", e);
            }
        });
    }
}


async fn serve_connection(tcp: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // We use the HTTP protocol to serve the connection. The `serve_connection` will use HTTP/1.
    let http = Http::new();
    /*
    Here is where we define which function/method called on a new http request    
    */
    let service = service_fn(handle_request);

    // Serve the connection
    if let Err(e) = http.serve_connection(tcp, service).await {
        error!("Error serving HTTP connection: {:?}", e);
        return Err(Box::new(e));
    }
    Ok(())
}