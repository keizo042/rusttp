use std::string;
use std::vec;
use std::hash;


struct Parser {
        body : &[u8],
        state : fn() -> usize,
        pos : usize,
        len : usize,
}

trait Parseable {
    fn new() -> self
    fn start(&mut self) -> ()
    fn emit() -> ()
    fn identify() -> ()
    fn header() -> ()
    fn hname() -> ()
    fn hvalue() -> ()
    fn body() -> ()
}

impl Parser for Parseable {
}
