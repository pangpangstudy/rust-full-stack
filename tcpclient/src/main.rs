use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    // 设置为可变
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    // write 需要可变引用：
    // 写操作可能会改变 TcpStream 的内部状态，比如更新缓冲区、改变连接状态等。
    // Rust 通过可变性来保证线程安全和防止数据竞争。
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!(
        "server to client message {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
