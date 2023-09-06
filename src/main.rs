use std::net::TcpListener;

fn main() {
   let tcp_listener = TcpListener::bind("127.0.0.1:4567").unwrap();

   for stream in tcp_listener.incoming() {
       let _stream = stream.unwrap();

       println!("Connection established!")
   }
}
