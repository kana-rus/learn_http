use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    io::{Read, Write}
};


mod user_code {
    use crate::{Request, Response, JSON};

    pub fn show_articles(_req: Request) -> Response {
        Response { headers: vec![], body: JSON() }
    }
    pub fn post_article(_req: Request) -> Response {
        Response { headers: vec![], body: JSON() }
    }
}

fn main() -> std::io::Result<()> {
    Server::setup()
        .GET("/", user_code::show_articles)
        .POST("/", user_code::post_article)
        .serve("127.0.0.1:3000")
}

struct Server(
    HashMap<
        &'static str,
        HashMap<Method, fn(Request) -> Response>,
    >,
); impl Server {
    fn setup() -> Self {
        Self(HashMap::new())
    }
    fn serve(&mut self, address: &'static str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; BUF_SIZE];

            stream.read(&mut buffer)?;
            let (path, method, request) = parse_stream(&buffer)?;

            println!("requested: {:?} {}", method,  path);

            let response = 'res: {
                let Some(handlers) = self.0.get(path) else {break 'res Response::NotFound()};
                let Some(handler) = handlers.get(&method) else {break 'res Response::NotFound()};
                handler(request)
            };
            stream.write(response.into_bytes())?;
            stream.flush()?
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    fn GET(&mut self,
        path:    &'static str,
        handler: fn(Request) -> Response,
    ) -> &mut Self {
        assert!(path.starts_with("/"));
        self.0
            .entry(path)
            .and_modify(|map|
                assert_eq!(None,
                    map.insert(Method::GET, handler),
                "handler for `GET {}` is already resistered", path)
            ).or_insert(
                HashMap::from([(Method::GET, handler)])
            );
        self
    }
    #[allow(non_snake_case)]
    fn POST(&mut self,
        path:    &'static str,
        handler: fn(Request) -> Response,
    ) -> &mut Self {
        assert!(path.starts_with("/"));
        self.0
            .entry(path)
            .and_modify(|map|
                assert_eq!(None,
                    map.insert(Method::POST, handler),
                "handler for `POST {}` is already resistered", path)
            ).or_insert(
                HashMap::from([(Method::GET, handler)])
            );
        self
    }
}

// struct TCPAddr(&'static str);
// fn on(addr: &'static str) -> TCPAddr {
//     TCPAddr(addr)
// }
#[derive(PartialEq, Eq, Hash, Debug)]
enum Method {
    GET,
    POST,
}

pub struct Request {
    headers: Vec<HeaderOfReq>,
    body:    Option<JSON>,
}
pub struct Response {
    headers: Vec<HeaderOfRes>,
    body:    JSON,
}
struct JSON();
enum HeaderOfReq {

}
enum HeaderOfRes {
    NotFoundMarkerForDebug
}

impl Response {
    fn into_bytes(&self) -> &[u8] {
        let mut serialized =
            if self.headers.is_empty() {
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\": true}"
            } else {
                "HTTP/1.1 404 NotFound\r\n"
            };
        // for header in self.headers {
        // 
        // }
        // self.body
        serialized.as_bytes()
    }
    #[allow(non_snake_case)]
    fn NotFound() -> Self {
        Self {
            headers: vec![HeaderOfRes::NotFoundMarkerForDebug],
            body:    JSON()
        }
    }
}

const BUF_SIZE: usize = 1024;
fn parse_stream(buffer: &[u8; BUF_SIZE]) -> std::io::Result<(&str, Method, Request)> {
    let request_status = {
        let mut end_of_reqest_status = BUF_SIZE;
        for pos in 0..BUF_SIZE {
            if buffer[pos]   == b'\r'  
            && buffer[pos+1] == b'\n' {
                if pos == 0 {
                    return Err(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "HTTP request starts with '\\r'"
                        )
                    )
                }
                end_of_reqest_status = pos - 1;
                break
            }
        }
        if end_of_reqest_status == BUF_SIZE {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HTTP request doesn't contain any valid request status",
                )
            )
        }
        &buffer[..=end_of_reqest_status]
    };

    let mut split = request_status.split(|b| *b == b' ');
    let method = match split.next().expect("no method found in request") {
        b"GET"  => Method::GET,
        b"POST" => Method::POST,
        _ => return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "HTTP request doesn't contain any valid method",
        ))
    };
    let path = std::str::from_utf8(
        split.next().expect("no request path found in request")
    ).expect("failed to get path from buffer");

    Ok((
        path,
        method,
        Request {
            headers: vec![],
            body:    None,
        }
    ))
}
