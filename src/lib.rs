pub mod endpoint;
pub mod clipboard_session;

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use crate::endpoint::{Endpoint};
    use std::net::{Ipv4Addr};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    fn _server_endpoint(listener: TcpListener) -> Endpoint {
        let (stream, _) = listener.accept().expect("No client found!");
        Endpoint::new_server(stream)
    }

    fn _client_endpoint(server_addr: SocketAddr) -> Endpoint {
        let stream = TcpStream::connect(server_addr).expect("Couldn't connect to server");
        Endpoint::new_client(stream)
    }

    #[test]
    fn test_server_write_client_read() {
        let mut rec_buf: [u8; 4] = [0; 4];
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let bound_addr = listener.local_addr().unwrap();

        let server_handle = thread::spawn(|| {
            let mut endpoint = _server_endpoint(listener);
            endpoint.write(&[1, 3, 3, 7]).unwrap();
        });
        let mut endpoint = _client_endpoint(bound_addr);
        server_handle.join().unwrap();
        endpoint.read(&mut rec_buf).unwrap();
        assert_eq!([1, 3, 3, 7], rec_buf)
    }

    #[test]
    fn test_client_write_server_read() {
        let mut rec_buf: [u8; 4] = [0; 4];
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let bound_addr = listener.local_addr().unwrap();

        let client_handle = thread::spawn(move || {
            let mut endpoint = _client_endpoint(bound_addr);
            endpoint.write(&[1, 3, 3, 7]).unwrap();
        });
        let mut endpoint = _server_endpoint(listener);
        client_handle.join().unwrap();
        endpoint.read(&mut rec_buf).unwrap();
        assert_eq!([1, 3, 3, 7], rec_buf)
    }
}