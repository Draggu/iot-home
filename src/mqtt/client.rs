use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use rumqttc::{AsyncClient, ClientError, Event, MqttOptions, Packet, Publish, QoS};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast::{self, Sender};
use tokio_stream::wrappers::BroadcastStream;
use tracing::{event, Level};

#[derive(Clone, Debug)]
pub struct MqttMessage {
    pub payload: String,
    pub topic: String,
}

#[derive(Debug)]
pub struct MqttError;

impl From<MqttError> for &str {
    fn from(_: MqttError) -> &'static str {
        "error on mqtt layer"
    }
}

impl From<ClientError> for MqttError {
    fn from(_: ClientError) -> Self {
        Self
    }
}

#[derive(Clone)]
/// very simple PubSub usig mqtt
pub struct MqttClient {
    client: AsyncClient,
    // topic to receivers
    // used topic will always be constant (without # and +)
    // so there is no need for more advanced routing
    txs: Arc<Mutex<HashMap<String, Sender<MqttMessage>>>>,
}

impl MqttClient {
    pub fn new(adress: &str, port: u16) -> Self {
        let (client, mut eventloop) =
            AsyncClient::new(MqttOptions::new("orchestrator", adress, port), 500);

        let txs = Arc::new(Mutex::new(HashMap::<String, Sender<MqttMessage>>::new()));
        let txs_clone = txs.clone();

        tokio::spawn(async move {
            loop {
                if let Ok(event) = eventloop.poll().await {
                    let message = format!("{:?}", event);
                    event!(Level::INFO, message);

                    if let Event::Incoming(Packet::Publish(Publish { topic, payload, .. })) = event
                    {
                        if let Some(tx) = txs.lock().await.get(&topic) {
                            let payload = String::from_utf8_lossy(&*payload).into_owned();

                            // error -> no receiver -> no subscriber -> just omit
                            tx.send(MqttMessage { topic, payload }).ok();
                        }
                    }
                }
            }
        });

        Self {
            client,
            txs: txs_clone,
        }
    }

    pub async fn subscribe<S: Into<String>>(
        &self,
        topic: S,
    ) -> Result<impl Stream<Item = MqttMessage>, MqttError> {
        let topic: String = topic.into();

        self.client
            .subscribe(topic.clone(), QoS::AtMostOnce)
            .await?;

        let mut guard = self.txs.lock().await;

        let tx = guard
            .entry(topic)
            .or_insert_with(|| broadcast::channel::<MqttMessage>(500).0);

        Ok(BroadcastStream::new(tx.subscribe()).map(Result::unwrap))
    }

    #[inline]
    pub async fn publish<S, V>(&self, topic: S, payload: V) -> Result<(), MqttError>
    where
        S: Into<String>,
        V: Into<String>,
    {
        Ok(self
            .client
            .publish(topic, QoS::AtMostOnce, true, payload.into())
            .await?)
    }
}
