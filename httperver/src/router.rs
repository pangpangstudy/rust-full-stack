use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
// 单元结构体（不包含任何字段）
pub struct Router;

impl Router {
    // 为 Router 实现一个 route 方法
    // 实现了 Write trait 的可变引用，用于写入响应，impl Write 允许这个方法接受任何实现了 Write trait 的类型，提高了灵活性
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        // 只处理Get请求
        match req.method {
            // 如果是 GET 方法，进一步匹配请求的资源。
            // &&req.resource 中的双引用  匹配模式
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // localhost  /  xxx/xxx/xxx
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
