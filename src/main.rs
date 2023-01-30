mod types;
mod db_functions;
mod handlers;
mod conf;
mod worker;

#[macro_use] extern crate rocket;

use db_functions::db_create;
use netlook::db_functions::db_truncate_calls_ip;
use worker::*;
use conf::load_config;
use clap::Parser;

use types::*;



use std::time::Duration;
use rocket_db_pools::Database;
use rocket_okapi::{ openapi_get_routes, swagger_ui::*};

use rocket::{fairing::AdHoc, fs::FileServer, routes};
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::Level::*;


#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Serve {
        #[clap(
            long,
            short,
            env = "RUSTETH_CONFIG"
        )]
        config: String
    }
}

#[launch]
async fn rocket() -> _ {
    let args = Args::parse();
    match args.subcmd.clone() {
        SubCommand::Serve {
            config
        } => { println!("args: {:0}", config);}
    }
    let cfg = load_config("config.json");

    Builder::new()
        .format(|buf, record| {
            let mut level_style = buf.style();
            match record.level() {
                Error => {level_style.set_color(env_logger::fmt::Color::Red).set_bold(true);},
                Info  => {level_style.set_color(env_logger::fmt::Color::Green).set_bold(true);},
                Warn  => {level_style.set_color(env_logger::fmt::Color::Yellow).set_bold(true);},
                Debug => {level_style.set_color(env_logger::fmt::Color::Blue).set_bold(true);},
                Trace => {level_style.set_color(env_logger::fmt::Color::White).set_bold(true);},
            };
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level_style.value(record.level()),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();


    tokio::spawn({
    let dbname = cfg.clone().dbname;
    let polling_duration = Duration::from_secs(cfg.api_call_timeout); 
    let tsharkfile = cfg.clone().tsharkfile;

    match db_create(&cfg.dbname){
        Ok(_) => {log::info!("DB created")}
        Err(e) => {log::warn!("DB creation failed: {e}")}
    }
    match db_truncate_calls_ip(&cfg.dbname){
        Ok(_) => {log::info!("Table cells truncated")}
        Err(e) => {log::warn!("Table cells truncation failed: {e}")}

    }
        async move {
            tshark_worker(polling_duration,dbname,tsharkfile).await;
        }
    });
    rocket::build()
                    .manage(cfg)
                    .mount("/", FileServer::from("./front/"))
                    .mount("/", openapi_get_routes![
                    handlers::common::getversion,

                    handlers::sending::get_calls_ip_layer,
                    handlers::sending::get_calls_calced

                    ])
                    .mount(
                        "/swagger/",
                        make_swagger_ui(&SwaggerUIConfig {
                            url: "../openapi.json".to_owned(),
                            ..Default::default()
                        }),
                    )
}
