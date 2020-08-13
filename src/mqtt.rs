use paho_mqtt::{
    Client, ConnectOptionsBuilder, CreateOptionsBuilder, MessageBuilder, MqttError, PersistenceType,
};
use std::time::Duration;

pub fn connect(mqtt_server: &str, file_persistence: bool) -> Result<Client, MqttError> {
    let create_options = CreateOptionsBuilder::new()
        .server_uri(mqtt_server)
        .persistence(if file_persistence {
            PersistenceType::File
        } else {
            PersistenceType::None
        })
        .finalize();

    let client = Client::new(create_options)?;

    let connection_options = ConnectOptionsBuilder::new()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .finalize();

    client.connect(connection_options)?;

    Ok(client)
}

pub fn publish(client: &Client, topic: &str, payload: &str, qos: i32) -> Result<(), MqttError> {
    let msg = MessageBuilder::new()
        .topic(topic)
        .qos(qos)
        .payload(payload)
        .finalize();

    client.publish(msg)
}
