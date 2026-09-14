use core::error;

use crate::resp::RespValue::BulkString;

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

fn parse_bulk_string(data : &[u8], start: usize) -> Result<(RespValue, usize), String> {
    let bulk_start = start;

    println!("Bulk starts at index: {}", bulk_start);
    println!("Bulk type byte: {:?}", data[bulk_start]);

    let bulk_length_end = data[bulk_start..]
                                .windows(2)
                                .position(|window| window == b"\r\n");

    match bulk_length_end {
        Some(length_end) => {
            let length_bytes = &data[bulk_start+1..bulk_start+length_end];

            let length_string = match std::str::from_utf8(length_bytes) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.to_string());
                }
            };

            let length = match length_string.parse::<usize>() {
                Ok(value) => value,
                Err(error) => {
                     return Err(error.to_string());
                }
            };

            let data_start = bulk_start+length_end+2;

            let bulk_data = &data[data_start..data_start+length];

            let bulk_string = RespValue::BulkString(bulk_data.to_vec());

            let next_position = data_start+length+2;

            Ok((bulk_string, next_position))
        }
        None => {
            Err("Bulk length CRLF not found".to_string())
        }
    }

                           
}

pub fn parse(data : &[u8]) -> Result<RespValue, String> {
    let first_byte = data[0];

    match first_byte {

        b'*' => {
            
            let position = data
                                            .windows(2)
                                            .position(|window| window==b"\r\n");
                            
            match position {
                Some(position) => {
                    let count_bytes = &data[1..position];

                    let count_string = match std::str::from_utf8(count_bytes){
                        Ok(value) => value,
                        Err(error) => {
                            return Err(error.to_string());
                        }
                    };

                    let count = match count_string.parse::<usize>(){
                        Ok(value) => value,
                        Err(error) =>{
                            return Err(error.to_string());
                        }
                    };

                    println!("Array count: {}", count);

                    let bulk_start = position+2;

                    let result = parse_bulk_string(data, bulk_start);

                    match result {
                        Ok((bulk_string, next_position)) => {
                            println!("Parsed bulk string: {:?}", bulk_string);
                            println!("Next position: {}", next_position);

                            let array = RespValue::Array(vec![bulk_string]);
                            Ok(array)
                        }

                        Err(error) => {
                            Err(error)
                        }
                    }

                }

                None =>{
                     println!("CRLF not found");
                     return Err("CRLF not found".to_string());
                }
            }

        }

        b'$' => {
            println!("This is a bulk string");
            return Err("Top-level bulk strings are not supported yet".to_string());
        }

        _ => {
            println!("Unknown RESP type");
            return Err("Unknown RESP type".to_string());
        }

    }
}