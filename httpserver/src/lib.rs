use std::net::TcpListener;

pub fn start_server(host: String, port: String) {
    let connection_string = format!("{}:{}", host, port);
    let listener = TcpListener::bind(connection_string).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
