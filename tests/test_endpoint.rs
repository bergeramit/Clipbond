#[cfg(test)]
mod tests {
    use clipbond::session::endpoint::{Endpoint, ConnectionInfo};
    use std::net::{Ipv4Addr};
    use std::{thread, time};

    #[test]
    fn test_server_write_client_read() {
        let mut rec_buf: [u8; 4] = [0; 4];
        let server_ip = Ipv4Addr::LOCALHOST;
        let server_port = 18344;

        let server_handle = thread::spawn( { 
            let clone_server_ip = server_ip;
            let clone_server_port = server_port;
            move || {
                let mut endpoint = Endpoint::new(ConnectionInfo::Server { listening_ip: clone_server_ip, listening_port: clone_server_port });
                endpoint.setup();
                endpoint.write(&[1, 3, 3, 7]).unwrap();
            }
        });

        let mut endpoint = Endpoint::new(ConnectionInfo::Client { server_ip, server_port });
        thread::sleep(time::Duration::from_millis(100));
        endpoint.setup();
        server_handle.join().unwrap();
        endpoint.read(&mut rec_buf).unwrap();
        assert_eq!([1, 3, 3, 7], rec_buf)
    }

    #[test]
    fn test_client_write_server_read() {
        let mut rec_buf: [u8; 4] = [0; 4];
        let server_ip = Ipv4Addr::LOCALHOST;
        let server_port = 18312;

        let client_handle = thread::spawn({
            let clone_server_ip = server_ip;
            let clone_server_port = server_port;
            move || {
                let mut endpoint = Endpoint::new(ConnectionInfo::Client { server_ip: clone_server_ip, server_port: clone_server_port });
                thread::sleep(time::Duration::from_millis(100));
                endpoint.setup();
                endpoint.write(&[1, 3, 3, 7]).unwrap();
            }
        });

        let mut endpoint = Endpoint::new(ConnectionInfo::Server { listening_ip: server_ip, listening_port: server_port });
        endpoint.setup();
        client_handle.join().unwrap();
        endpoint.read(&mut rec_buf).unwrap();
        assert_eq!([1, 3, 3, 7], rec_buf)
    }
}