use::std::net::TcpListener;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("Velocity server started on port 6379");

    loop{
        let (stream, address) = listener.accept().unwrap();
        println!("Client connected, {}", address);
    }
}