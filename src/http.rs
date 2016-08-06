use std::net::TcpStream;
use std::result::Result;
use std::io::{Write,Read};
use std::string::String;

pub fn send(dist: String, port: u64) -> String
{
    let mut stream = TcpStream::connect(format!("{}:{}",dist, port).as_str()).unwrap();

    let _ = stream.write(format!("{}{}{}{}", "GET / HTTP/1.1\nHost", dist.as_str(), "Connection:close\n" , "\n").as_bytes());
    let mut s = String::with_capacity(4096);
    let mut t = stream.take(4096);
    let _ = t.read_to_string(&mut s);
    return s;
}

pub fn connect(dist :String, port: u64)
{
}
    

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
