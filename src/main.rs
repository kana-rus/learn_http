use std::{collections::HashMap, net::{TcpListener, TcpStream}, io::Read};


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
    fn serve(&self, address: &'static str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let (path, method, request) = parse_stream(stream)?;
            Self::handle(path, method, request)
        }
        Ok(())
    }
    fn handle(path: [u8; 256], method: Method, request: Request) {
        
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
            ).or_insert(HashMap::new());
        self
    }
    #[allow(non_snake_case)]
    fn POST(&mut self,
        path:    &'static str,
        handler: fn(Request) -> Response,
    ) -> &mut Self {
        assert!(path.starts_with("/"));
        self.0.entry(path)
            .and_modify(|map|
                assert_eq!(None,
                    map.insert(Method::POST, handler),
                "handler for `POST {}` is already resistered", path)
            ).or_insert(HashMap::new());
        self
    }
}

// struct TCPAddr(&'static str);
// fn on(addr: &'static str) -> TCPAddr {
//     TCPAddr(addr)
// }
#[derive(PartialEq, Eq, Hash)]
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

}

const BUF_SIZE: usize = 1024;
fn parse_stream(stream: TcpStream) -> std::io::Result<([u8; 256], Method, Request)> {
    let mut buff = [0u8; BUF_SIZE];
    stream.read(&mut buff)?;

    let request_status = {
        let mut end_of_reqest_status = BUF_SIZE;
        for pos in 0..BUF_SIZE {
            if buff[pos]   == b"\r"[0]  
            && buff[pos+1] == b"\n"[0] {
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
        &buff[..=end_of_reqest_status]
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
    let mut path = [b' '; 256];
    split.next().expect("no request path found in request").read(&mut path)?;

    Ok((
        path,
        method,
        Request {
            headers: vec![],
            body:    None,
        }
    ))
}
