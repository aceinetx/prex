use serde_json::{Map, Value};

pub const SOCKET_PATH: &str = "/tmp/prex.sock";

pub const PACKET_SHUTDOWN: i8 = 1;
pub const PACKET_EXEC: i8 = 2;
pub const PACKET_INFO: i8 = 3;

pub fn construct_shutdown_packet() -> String {
    let mut packet = Map::new();
    packet.insert("type".to_string(), Value::Number(PACKET_SHUTDOWN.into()));

    return serde_json::to_string(&Value::Object(packet)).unwrap();
}

pub fn construct_info_packet() -> String {
    let mut packet = Map::new();
    packet.insert("type".to_string(), Value::Number(PACKET_INFO.into()));

    return serde_json::to_string(&Value::Object(packet)).unwrap();
}

pub fn construct_exec_packet(argv: Vec<String>) -> String {
    let mut packet = Map::new();
    packet.insert("type".to_string(), Value::Number(PACKET_EXEC.into()));
    packet.insert("args".to_string(), serde_json::to_value(argv).unwrap());

    return serde_json::to_string(&Value::Object(packet)).unwrap();
}
