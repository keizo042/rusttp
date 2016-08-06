use std::string;
use std::vec;
use std::hash;


struct Parser {
        body :& [u8],
        state : usize,
        pos : usize,
        len : usize,
}

trait Parser {

    pub fn start() {
    }

    fn header(mut& self) {
    }

    fn body(mut& self) {
    }

}
