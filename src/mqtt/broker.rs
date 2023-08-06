use rumqttd::{
    protocol::v4::V4, Broker, ConnectionSettings, LinkType, RouterConfig, Server, ServerSettings,
};
use std::net::{IpAddr, SocketAddr};

/// this works on patched version of rumqttd
/// patch allows using broker in current runtime
/// more details can be found in [patch/rumqttd]
#[inline]
pub async fn create(host: IpAddr, port: u16) {
    let mut config = rumqttd::Config::default();

    config.id = 0;

    config.router = RouterConfig {
        instant_ack: true,
        max_connections: 10010,
        max_read_len: 10240,
        max_segment_count: 10,
        max_segment_size: 104857600,
        initialized_filters: None,
    };

    let server_settings = ServerSettings {
        connections: ConnectionSettings {
            connection_timeout_ms: u16::MAX,
            dynamic_filters: true,
            max_inflight_count: u16::MAX,
            max_inflight_size: usize::MAX,
            max_payload_size: usize::MAX,
            throttle_delay_ms: 0,
            auth: None,
        },
        listen: SocketAddr::new(host, port),
        name: "v4".to_owned(),
        next_connection_delay_ms: 0,
        tls: None,
    };

    let broker = Broker::new(config);

    let broker_server = Server::new(server_settings, broker.router_tx.clone(), V4);

    broker_server.start(LinkType::Remote).await.unwrap();
}
