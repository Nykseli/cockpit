#[derive(Debug)]
pub enum BridgeMessage {
    Text(String),
    // TODO:
    #[allow(dead_code)]
    Binary(Vec<u8>),
}
