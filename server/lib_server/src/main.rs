#[macro_use]
extern crate rocket;

mod constants;
mod service_data;
mod struct_db;
mod struct_auth;
mod struct_config;
mod struct_group;
mod struct_label;
mod struct_response;
mod struct_server;
mod struct_task;
mod struct_user;

mod route_auth;
mod route_users;
mod route_groups;
// mod route_bases;
mod route_templates;
mod route_associates;
mod route_items;
mod route_labels;
// mod route_logs;
// mod route_tasks;

use num_cpus;
use std::env;
use std::time::Duration;
use std::convert::From;
use std::fs::create_dir_all;
use std::net::IpAddr;
use rumqttc::{MqttOptions, AsyncClient, QoS, EventLoop};
use rocket::config::{Config as ConfigRocket, LogLevel};
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use constants::*;
use struct_config::Config;
use struct_db::DB;
use struct_server::Server;
use struct_auth::Auth;

#[rocket::main]
async fn main() {

    /* config */
    let config: Config = Config::load(&CONFIG_PATH);

    /* log */
    create_dir_all(&config.log).unwrap();
    fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{} [{}][{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S:%MS"),
            record.level(),
            record.target(),
            message
        ))
    })
    .level(log::LevelFilter::Info)
    .chain(std::io::stdout())
    .chain(fern::DateBased::new(&config.log, "%Y-%m-%d"))
    .apply()
    .unwrap();
    
    /* db */
    let db: DB = DB::new(&config.db);

    /* mqtt
        "TOPIC_REFRESH_LIST":"test/refresh/queue",
        "TOPIC_NOTIFY":"test/refresh/notify",
        "TOPIC_CONFIG":"test/device/config",
        "TOPIC_DEVICE_PROPERTY":"test/device/property"
        TODO: migrate to lib_vendors
    */
    let mut mqttoptions = MqttOptions::new("server", &config.mqtt_host, config.mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    let (mqtt, mut _eventloop): (AsyncClient, EventLoop) = AsyncClient::new(mqttoptions, 10);
    mqtt.subscribe("test/refresh/queue", QoS::AtMostOnce).await.unwrap();
    rocket::tokio::spawn(async move {
        while let Ok(_notification) = _eventloop.poll().await {
            // TODO
            // match notification {
            //     Event::Incoming(e) => {
            //         info!("<= mqtt {:?}", e);
            //     },
            //     Event::Outgoing(e) => {
            //         info!("=> mqtt {:?}", e);
            //     },
            // }
        }
    });

    /* state */
    let server: Server = Server::new(db, mqtt, config.images.clone());
    { server.cache.lock().unwrap().put(TOKEN_USER.to_string() , Auth::_mock_user()); }

    /* brand */
    info!("Reducing Technologies - {}", env!("CARGO_PKG_VERSION"));

    /* http */
    let mut config_rocket = ConfigRocket::default();
    config_rocket.address = IpAddr::V4(config.address.parse().unwrap());
    config_rocket.port = config.port;
    config_rocket.workers = num_cpus::get() * 2;
    config_rocket.log_level = LogLevel::Off;

    /* cors */
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[
            "http://localhost:8080",
            "https://na0.reducing.ca",
        ]),
        allowed_methods: vec![Method::Post, Method::Get, Method::Delete, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Content-Type", "Authorization"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    /* init */
    let _ = rocket::custom(config_rocket)
        .attach(cors)
        .mount(
            "/api/groups",
            routes![

                // groups
                route_groups::post, route_groups::patch, route_groups::get, route_groups::delete, route_groups::summary,

                // items
                route_items::post, route_items::get, route_items::delete,

                // labels
                route_labels::post, route_labels::get, route_labels::delete,

                // templates
                route_templates::post, route_templates::get, route_templates::delete,

                // associates
                route_associates::post, route_associates::get, route_associates::delete,

                // bases
                // route_bases::post, route_bases::get, route_bases::delete,
            ],
        )
        .mount(
            "/api",
            routes![

                // logs
                // route_logs::get,

                // tasks
                // route_tasks::get, route_tasks::get_summary,

                // users
                route_users::post, route_users::patch_password, route_users::patch_role, route_users::delete, route_users::get,

                // auth
                route_auth::post,
            ],
        )
        .mount("/", FileServer::from("public"))
        .manage(server)
        .launch()
        .await;
}