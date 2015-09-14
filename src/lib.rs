extern crate hyper;

use std::net::{ SocketAddr, Ipv4Addr, SocketAddrV4 };

use hyper::server::{ Request };
use hyper::uri::RequestUri;
use hyper::header::Headers;
use hyper::http::h1::HttpReader;
use hyper::method::{ Method };
use hyper::version::{ HttpVersion };
use hyper::http::h1::HttpReader::{SizedReader, ChunkedReader, EmptyReader};

#[test]
fn construct_request() {
    let req : Request = Request {
        remote_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0u8,0u8,0u8,0u8), 0u16)),
        method: Method::Get,
        headers: Headers::new(),
        uri: RequestUri::AbsolutePath(String::from("/hello/world")),
        version: HttpVersion::Http20,
        body: MockStream()
    };
}
