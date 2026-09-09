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
                            
            match position {
                Some(position) => {
                    let count_bytes = &data[1..position];

                    let bulk_start = position+2;

                    println!("Bulk starts at index: {}", bulk_start);
                    println!("Bulk type byte: {:?}", data[bulk_start]);

                    let bulk_length_end = data[bulk_start..]
                                              .windows(2)
                                              .position(|window| window == b"\r\n" );

                    match bulk_length_end {
                          Some(length_end) => {
                            let length_bytes = &data[bulk_start+1..bulk_start+length_end];
                            println!("Bulk length bytes: {:?}", length_bytes);
                          }

                          None => {
                            println!("Bulk length CRLF not found");
                          }
                    };

                    let count_string = match std::str::from_utf8(count_bytes){
                        Ok(value) => value,
                        Err(error) => {
                            println!("Invalid UTF-8: {}", error);
                            return;
                        }
                    };

                    let count = match count_string.parse::<usize>() {
                        Ok(value) => value,
                        Err(error) => {
                            println!("Invalid array count: {}", error);
                            return;
                        }
                    };

                    println!("Array count: {}", count);
                }

                None =>{
                     println!("CRLF not found");
                }
            }

        }

        b'$' => {
            println!("This is a bulk string");
        }

        _ => {
            println!("Unknown RESP type");
        }

    }
}