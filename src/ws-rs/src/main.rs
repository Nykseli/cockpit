use std::fs;

use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use clap::Parser;

mod cli;
mod cockpit_branding;
use self::cockpit_branding::cockpit_static;
mod server;
use self::server::MyWebSocket;
mod state;
use self::state::CockpitState;
use self::state::WebCockpitState;
mod constants;
use self::constants::STATIC_BASE_PATH;

#[get("/")]
async fn index(data: WebCockpitState) -> Result<HttpResponse, Error> {
    let html_base = fs::read_to_string(format!("{STATIC_BASE_PATH}login.html"))?;
    let enviroment = data.build_js_environment();
    Ok(HttpResponse::Ok()
        .body(html_base.replace("<meta insert_dynamic_content_here>", &enviroment)))
}

#[get("/cockpit/login")]
async fn login() -> HttpResponse {
    // TODO: handle login src/ws/cockpithandlers.c:cockpit_handler_default
    HttpResponse::Unauthorized().body("You cannot login yet!")
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = cli::Args::parse();
    let port = &args.get_port();
    let address = &args.get_address();

    log::info!("starting HTTP server at http://{address}:{port}",);

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(CockpitState::new()))
            // .service(web::resource("/").to(index))
            .service(index)
            // websocket route
            .service(web::resource("/cockpit/socket").route(web::get().to(echo_ws)))
            .service(cockpit_static)
            .service(login)
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((address.to_owned(), port.to_owned()))?
    .run()
    .await
}
