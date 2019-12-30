extern crate actix_rt;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod parse;

use actix_web::{middleware, web, App, HttpServer};
use std::env;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Result as IoResult;
use std::str::FromStr;

fn is_parser_in_path() -> bool {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(":") {
            let program_path = format!("{}/mercury-parser", p);

            if std::fs::metadata(program_path).is_ok() {
                return true;
            }
        }
    }

    false
}

#[actix_rt::main]
async fn main() -> IoResult<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting oxi version 1.0.0");

    let args: Vec<String> = env::args().collect();
    let mut port: u16 = 8080;

    if args.len() == 3 {
        port = u16::from_str(&args[2]).unwrap();
    }

    if is_parser_in_path() == false {
        let err_string = "Can't find `mercury-parser` in $PATH; aborting.";
        return Err(IoError::new(ErrorKind::NotFound, err_string));
    }

    let app = move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/parse").route(web::post().to(parse::parse)))
    };

    HttpServer::new(app)
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
