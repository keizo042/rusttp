use std::net::TcpStream;
use std::result::Result;
use std::io::{Write,Read};
use std::io;
use std::string::String;
use std::vec::Vec;

struct Conn{
    body : String,
}

pub fn send(dist: String, port: u64) -> Option<String>{
    let mut stream = match TcpStream::connect(format!("{}:{}",dist, port).as_str()) {
        Ok(v) =>  v,
        Err(e) => return None,
    };

    let t = format!("{}{}{}{}", "GET / HTTP/1.1\nHost", dist.as_str(), "Connection:close\n" , "\n");
    let _ = stream.write(t.as_bytes());
    let mut v: Vec<u8> = Vec::new();
    let _ = stream.read_to_end(&mut v);
     match String::from_utf8(v) {
        Ok(v) => return Some(v),
        Err(e) => return None,
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
