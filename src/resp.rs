#[derive(Debug)]
pub enum RespValue{
    BulkString(Vec<u8>),
    Array(Vec<RespValue>)
}

pub fn ping() -> RespValue {
    let ping_string = RespValue::BulkString(b"PING".to_vec());
    let array = RespValue::Array(vec![ping_string]);
    array
}

pub fn encode_bulk_string(value: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();

    encoded.push(b'$');
    encoded.extend_from_slice(
        value.len().to_string().as_bytes()
    );
    encoded.extend_from_slice(b"\r\n");

    encoded
}