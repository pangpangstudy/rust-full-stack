use std::collections::HashMap;
// #[...]是Rust 中的属性语法，属性用于向编译器提供额外的信息或指令
// derive 这是一个特殊的属性，用于自动生成特定 trait 的实现。它告诉编译器为标记的类型自动实现指定的 traits
// Debug 这是 std::fmt::Debug trait，实现这个 trait 允许使用 {:?} 格式说明符来格式化和打印该类型的值。对于调试非常有用，可以轻松打印复杂的数据结构。
// PartialEq std::cmp::PartialEq trait ， 实现这个 trait 允许使用 == 和 != 运算符来比较该类型的值
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
// 由于 From 是标准库的一部分并且在 prelude 中，我们可以直接使用它而无需引入。
// From 是一个泛型 trait，定义为 trait From<T>，其中 T 是源类型
// 在这个实现中，我们明确指定了 T 为 &str
// 表示我们正在为 Method 类型实现 From trait，这个实现专门用于从 &str 类型转换。
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            r"HTTP\1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Resource {
    //  Path(String) 是 Rust 中枚举（enum）的一种变体（variant）定义方式，具体称为元组变体（tuple variant）。
    //  Path 是这个变体的名称,(String) 表示这个变体包含一个 String 类型的数据
    // 这种形式的变体被称为元组变体，因为它类似于一个只有一个元素的元组
    // let resource = Resource::Path("example.txt".to_string());
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    // HashMap 会在需要时自动增长和重新哈希
    // 标准的 HashMap 不是线程安全的。对于并发场景，可以使用 std::sync::RwLock<HashMap> 或第三方库如 dashmap。
    // HashMap 在堆上分配内存，可能比数组或向量使用更多内存
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        // 初始化 变量
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key, value)
}

// 这是一个条件编译属性。它告诉 Rust 编译器只在运行测试时编译这个模块,在正常的程序构建中，这个模块会被忽略。
#[cfg(test)]
// 这定义了一个名为 tests 的模块,在 Rust 中，通常将测试代码放在一个单独的模块中
mod tests {
    // super 关键字指的是父模块,* 表示导入父模块中的所有项
    use super::*;
    // 这个属性标记下面的函数为一个测试函数。
    #[test]
    fn test_method_into() {
        // .into() 方法尝试将 &str 转换为 Method。这里利用了类型推断和 Into trait（它是 From 的反向 trait）
        // 实现原理:当你实现 From<&str> for Method：Rust 自动为 &str 实现了 Into<Method>。将&str转换为Method
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into() {
        let v: Version = r"HTTP\1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser_Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected: HashMap<String, String> = HashMap::new();
        //
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
    }
}
// Into 是 Rust 标准库中的一个 trait。它定义在 std::convert::Into 中。它是 From trait 的对偶（dual）
// From 和 Into 的关系:当你为类型 A 实现 From<B>，Rust 自动为 B 实现 Into<A>。这意味着你通常只需要实现 From，就能同时得到 Into 的功能。
// let s: String = "hello".into(); // &str 转 String
