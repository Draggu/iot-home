use async_graphql::Schema;
use async_graphql_axum::GraphQLSubscription;
use axum::{routing::post, Extension, Router, Server};
use dotenv::dotenv;
use gql::{graphql_handler, Mutation, Query, Subscription};
use std::env;

mod device_registry;
mod devices;
mod gql;
mod mqtt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mqtt_hostname = env::var("MQTT_HOSTNAME").unwrap_or("127.0.0.1".to_owned());
    let mqtt_port = env::var("MQTT_PORT")
        .map(|port_string| {
            port_string
                .parse::<u16>()
                .expect("MQTT_PORT must be u16 compatible string")
        })
        .unwrap_or(1883);

    let mqtt_hostname_clone = mqtt_hostname.clone();

    let broker_handle =
        tokio::spawn(async move { mqtt::broker::create(mqtt_hostname_clone, mqtt_port).await });

    broker_handle.abort();

    let mqtt = mqtt::client::MqttClient::new(mqtt_hostname, mqtt_port);

    let devices = device_registry::device::DevicesManager::load(
        env::var("DEVICES_FILE").unwrap_or("./devices.json".to_owned()),
    );

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(mqtt)
    .data(devices)
    .finish();

    let app = Router::new()
        .route("/gql", post(graphql_handler))
        .route_service(
            "/gql/subscriptions",
            GraphQLSubscription::new(schema.clone()),
        )
        .layer(Extension(schema))
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
