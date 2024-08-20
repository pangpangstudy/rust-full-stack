use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    // 创建监听器
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("running on port 3000...");
    for stream in listener.incoming() {
        // 取出值
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        // 流处理的是buffer 二进制
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
