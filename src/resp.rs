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
    encoded.extend_from_slice(value);
    encoded.extend_from_slice(b"\r\n");

    encoded
}

pub fn encode_array(values: &[RespValue]) -> Vec<u8> {
    let mut encoded = Vec::new();
    encoded.push(b'*');
    encoded.extend_from_slice(values.len().to_string().as_bytes());
    encoded.extend_from_slice(b"\r\n");

    for value in values {
        match value {
            RespValue::BulkString(data) => {
                let bulk_string = encode_bulk_string(data);
                encoded.extend_from_slice(&bulk_string);
            }
            RespValue::Array(values)=>{
                let array = encode_array(values);
                encoded.extend_from_slice(&array);
            }
        }
    }

    encoded
}

pub fn parse(data : &[u8]) {
    let first_byte = data[0];

    match first_byte {

        b'*' => {
            
            let position = data
                                            .windows(2)
                                            .position(|window| window==b"\r\n");
                            
            println!("CRLF position: {:?}", position);

        }

        b'$' => {
            println!("This is a bulk string");
        }

        _ => {
            println!("Unknown RESP type");
        }

    }
}