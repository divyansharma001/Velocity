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