use std::collections::HashMap;
use std::io::{Result, Write};
// 任何引用类型都需要生命周期标注。
// 拥有所有权的类型（如 String, Vec 等）不需要生命周期标注。
// 结构体中有引用，整个结构体就需要生命周期参数。
// impl 块和方法中使用的生命周期要与结构体定义一致。
#[derive(Debug, PartialEq, Clone)]
// 当结构体中的 字段是引用类型 需要添加生命周期
// 对于拥有所有权的类型（如 String），不需要生命周期标注
pub struct HttpResponse<'a> {
    // 不需要修改所以用了 引用
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    // body 是 Option<String>，String 拥有所有权，不需要生命周期标注
    body: Option<String>,
}
// 当为带有生命周期参数的结构体实现方法时，需要在 impl 后声明生命周期。
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}
// 为特定类型实现from
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}
// 当为带有生命周期参数的结构体实现方法时，需要在 impl 后声明生命周期。
// 如果方法参数或返回值涉及结构体的生命周期，需要使用相同的生命周期标注
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        // 初始化变量
        let mut response: HttpResponse<'a> = HttpResponse::default();
        // 状态码
        if status_code != "200" {
            // 直接赋值 status_code 是可以的，因为两者都是 &'a str 类型 response.status_code = status_code;
            // 它提供了更好的灵活性。如果将来 status_code 的类型改变（比如改为 String），.into() 仍然可以工作。
            // 它使代码更加一致，特别是当你在其他地方也使用 .into() 时
            response.status_code = status_code.into();
        }
        // header
        response.headers = match &headers {
            // 有值就返回值
            // 这里使用 _h 是一种常见的 Rust 模式匹配写法
            // 这个模式匹配 Some 变体，但我们不需要使用其中的值
            // 它只是检查 headers 是否是 Some，而不关心 Some 中具体包含什么
            Some(_h) => headers,
            // 没值 就创建一个
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        // 返回status_text 根据状态码 设置
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        // 返回body
        response.body = body;
        response
    }
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        // clone() 是 Rust 中用于创建对象深拷贝的方法。创建一个对象的完整副本，包括所有拥有的数据,新副本与原对象完全独立，修改一个不会影响另一个,对于复杂的数据结构，可能会涉及大量的内存分配和复制。
        // 实现了 Clone trait 的类型才能使用 clone()
        let res = self.clone();
        let response_string: String = String::from(res);
        // write! 是 Rust 标准库提供的一个宏，用于格式化并写入数据到一个实现了 std::io::Write trait 的对象中
        // 语法 write!(destination, "formatted string {}", value)
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    // getter
    fn version(&self) -> &str {
        // 方法返回一个对 self.status_text 的引用,不转移所有权，只是借用数据
        // 适用于 status_text 字段本身就是 &str 类型的情况,生命周期与 &self 相关联，意味着返回的引用不能比 self 活得更久
        &self.version
    }
    fn status_code(&self) -> &str {
        &self.status_code
    }
    fn status_text(&self) -> &str {
        &self.status_text
    }
    fn headers(&self) -> String {
        // unwrap() 是 Rust 中常用但需谨慎使用的方法。它主要用于处理 Option 和 Result 类型
        // 有值取值 None 直接panic
        // unwrap_or(default): 提供一个默认值，在 None 或 Err 时返回。
        // unwrap_or_else(f): 提供一个闭包，在 None 或 Err 时调用。
        // expect("message"): 类似 unwrap()，但可以指定 panic 时的错误消息。
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    fn body(&self) -> String {
        match &self.body {
            Some(body) => body.into(),
            None => "".into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx"
                .to_string();
        assert_eq!(http_string, actual_string);
    }
}
