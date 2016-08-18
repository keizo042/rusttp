use std::net::TcpStream;
use std::string::String;
use std::io::{Read,Write};
use std::vec::Vec;

struct Conn{
    body : String,
}

pub fn send(dist: String, port: u64) -> Option<String>{
    let mut addr = dist.clone();
    addr.push_str(":");
    addr.push_str(port.to_string().as_str());
    let mut stream = match TcpStream::connect(addr.as_str()) {
        Ok(v) =>  v,
        Err(_) => {return None}
    };

    let mut t = String::new();
    t.push_str("GET / HTTP/1.1\n");
    t.push_str("Host:");
    t.push_str(dist.as_str());
    t.push_str("\n");
    t.push_str("Connction:closed\n");
    t.push_str("\n");

    let _ = stream.write(t.as_bytes());
    let mut v: Vec<u8> = Vec::new();
    let _ = stream.read_to_end(&mut v);
     match String::from_utf8(v) {
        Ok(v) => return Some(v),
        Err(_) => return None,
    };
}

#[cfg(test)]
mod tests {
    use http;
    #[test]
    fn send_to_google() {
        let v = match http::send("google.com".to_string(),80) {
            Some(v) =>v,
            None => "".to_string(),
        };
        let text = r#"HTTP/1.1 302 Found
        Cache-Control: private
        Content-Type: text/html; charset=UTF-8
        Location: http://www.google.co.jp/?gfe_rd=cr&ei=hwymV5-HOOOQ8Qel6aMw
        Content-Length: 259
        Date: Sat, 06 Aug 2016 16:12:55 GMT
        Connection: close

        <HTML><HEAD><meta http-equiv="content-type" content="text/html;charset=utf-8">
        <TITLE>302 Moved</TITLE></HEAD><BODY>
        <H1>302 Moved</H1>
        The document has moved
        <A HREF="http://www.google.co.jp/?gfe_rd=cr&amp;ei=hwymV5-HOOOQ8Qel6aMw">here</A>.
        </BODY></HTML>"#;
        assert_eq!(text,v.to_string());

    }
}
