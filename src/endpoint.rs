use std::io::{Write, Read};
use std::net::{TcpStream};

pub struct Endpoint {
    stream: TcpStream,
}

impl Endpoint {
    pub fn new(stream: TcpStream) -> Self {
        return Endpoint {
            stream
        }
    }
    
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.write(buf)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.read(buf)
    }
}
