extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{
    Nickel, Request, Response, HttpRouter
};
use std::io::net::ip::Ipv4Addr;
use std::sync::atomic::{AtomicUint, Relaxed};

struct Logger {
    visits: AtomicUint
}

fn main() {
    let mut server = Nickel::new();

    fn root_handler(_request: &Request, response: &mut Response, logger: &Logger) {
        response.send(format!("{}", logger.visits.fetch_add(1, Relaxed)));
    }

    server.get("/", (root_handler, Logger{visits: AtomicUint::new(0)}));
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
