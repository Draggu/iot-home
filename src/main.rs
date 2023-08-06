use async_graphql::Schema;
use async_graphql_axum::GraphQLSubscription;
use axum::{routing::post, Extension, Router, Server};
use devices::{registry::service::DeviceService, specials::voltage::service::VoltageService};
use dotenv::dotenv;
use gql::{graphql_handler, Mutation, Query, Subscription};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::env;
use tower_http::cors::CorsLayer;
use tracing::subscriber;
use tracing_subscriber::FmtSubscriber;

mod db;
mod devices;
mod gql;
mod migrations;
mod mqtt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    subscriber::set_global_default(FmtSubscriber::new()).expect("setting tracing default failed");

    let db = Database::connect("sqlite://db.sqlite?mode=rwc")
        .await
        .unwrap();

    migrations::migrator::Migrator::up(&db.clone(), None)
        // migrations::migrator::Migrator::fresh(&db.clone())
        .await
        .unwrap();

    let mqtt_hostname = env::var("MQTT_HOSTNAME").unwrap_or("127.0.0.1".to_owned());
    let mqtt_port = env::var("MQTT_PORT")
        .map(|port_string| {
            port_string
                .parse::<u16>()
                .expect("MQTT_PORT must be u16 compatible string")
        })
        .unwrap_or(1883);

    tokio::spawn(mqtt::broker::create(
        mqtt_hostname.parse().unwrap(),
        mqtt_port,
    ));

    let mqtt = mqtt::client::MqttClient::new(&mqtt_hostname, mqtt_port);

    let devices = DeviceService::new(db.clone());

    let voltage = VoltageService::new(db.clone(), devices.clone(), mqtt.clone());

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(mqtt)
    .data(devices)
    .data(db)
    .data(voltage)
    .finish();

    let app = Router::new()
        .route("/gql", post(graphql_handler))
        .route_service(
            "/gql/subscriptions",
            GraphQLSubscription::new(schema.clone()),
        )
        .layer(Extension(schema))
        .layer(CorsLayer::very_permissive())
        .into_make_service();

    let _server = Server::bind(
        &env::var("WEB_HOST")
            .unwrap_or("127.0.0.1:8080".to_owned())
            .parse()
            .unwrap(),
    )
    .serve(app)
    .await;
}
