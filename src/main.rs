mod resp;
use std::error;
use::std::net::TcpListener;
use::std::io::{Read, Write};

fn main(){

    let encoded = resp::encode_bulk_string(b"PING");
    println!("{:?}", encoded);
    
    let listener = match TcpListener::bind("127.0.0.1:6379"){
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Failed to bind to 127.0.0.1:6379: {}", error);
            return;
        }
    };

    println!("Server is listening on 127.0.0.1:6379");

    loop{
        let (mut stream, address) = match listener.accept(){
            Ok(connection) => connection,
            Err(error) => {
                eprintln!("Failed to accept connection {}", error);
                continue;
            }
        };
        
        println!("Client connected: {}", address);

        let mut buffer = [0u8; 1024];

        loop{

        let bytes_read = match stream.read(&mut buffer){
            Ok(0) => {
                println!("Client Disconnected");
                 break;
            },

            Ok(bytes) => bytes,

            Err(error) => {
                eprintln!("Failed to read from the client: {}", {error});
                break;
            }
        };

        println!("Recieved {} bytes", bytes_read);
        println!("Raw bytes: {:?}", &buffer[..bytes_read]);

      match stream.write_all(&buffer[..bytes_read]) {
            Ok(()) => {
                println!("Response sent");
            }

            Err(error) => {
                eprintln!("Failed to write to the client: {}", error);
                break;
            }
        };

    }

    }
}