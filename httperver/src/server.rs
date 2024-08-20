// use super::router::Router;
use http::httprequest::HttpRequest;
use std::{io::prelude::*, net::TcpListener};

use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        // 取出stream
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            // 访问数据存入
            let mut buffer = [0; 1024];
            // 访问数据写入
            stream.read(&mut buffer).unwrap();
            // 字符串反向推断为 HttpRequest
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            // 使用req 和 流的引用  调用router
            Router::route(req, &mut stream);
        }
    }
}
