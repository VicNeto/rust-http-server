use server::Server;

mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}