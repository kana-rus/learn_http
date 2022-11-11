mod parser;
mod error;

use crate::http1::{Method, RequestUri, MessageBody, HeaderInRequest, utils::consts::BUFF_SIZE};
use self::error::ParseError;


struct Request {
    method:  Method,
    uri:     RequestUri,
    headers: Vec<HeaderInRequest>,
    body:    Option<MessageBody>,
}

fn parse_response(buf: &[u8]) -> Result<Request, ParseError> {
    let mut buf_read_pos = 0;

    let request_line = read_line(buf, &mut buf_read_pos)?;
    let (method, uri, version) = split_sp_2(request_line)?;
    if version != b"HTTP/1.1" {
        return Err(ParseError::InvalidVersion(format!(
            "Requested HTTP-Version '{}' is invalid. I accept only HTTP/1.1.",
            String::from_utf8_lossy(version)
        )))
    }
    let method = match method {
        b"OPTIONS" => Method::Options,
        b"GET"     => Method::Get,
        b"HEAD"    => Method::Head,
        b"POST"    => Method::Post,
        b"PUT"     => Method::Put,
        b"DELETE"  => Method::Delete,
        b"TRACE"   => Method::Trace,
        b"CONNECT" => Method::Connect,
        _ => return Err(ParseError::ReadLineError(format!(
            "Invalid method requested: {}",
            String::from_utf8_lossy(method)
        )))
    };
    let uri = match method {
        Method::Connect => RequestUri::Authority(String::from_utf8_lossy(uri).into()),
        _ => {
            if uri[0] == b'*' {
                RequestUri::Asterisk
            } else if uri[0] == b'/' {
                RequestUri::AbsPath(String::from_utf8_lossy(uri).into())
            } else {
                RequestUri::AbsoluteUri({
                    let cow = String::from_utf8_lossy(uri);
                    if &uri[0..=6] != b"http://" {
                        return Err(ParseError::InvalidUri(format!(
                            "Request-URI token is invalid: {}",
                            cow
                        )))
                    }
                    cow.into()
                })
            }
        }
    };

    
    

}

fn read_line<'b>(buf: &'b [u8], current_pos: &mut usize) -> Result<&'b [u8], ParseError> {
    for eol in *current_pos..BUFF_SIZE-1 {
        if buf[eol]   == b'\r'
        && buf[eol+1] == b'\n' {
            let line = &buf[*current_pos..eol];
            *current_pos = eol + 2;
            return Ok(line)
        }
    }
    Err(ParseError::ReadLineError(format!(
        "Passed buffer expected to be a line doesn't contain CRLF. Buffer content: {}",
        String::from_utf8_lossy(buf)
    )))
}
fn split_sp_2(line: &[u8]) -> Result<(&[u8], &[u8], &[u8]), ParseError> {
    let eol = line.len();
    let (mut sp_pos_1, mut sp_pos_2) = (0, 0);
    
    for pos in 1..eol-3 {
        /*
            o o o o ...  o   o   o  eol
            0 1 2 3 ... -3  -2  -1  len
                         |--> これ以降に sp_pos_1 があると 3parts になりようがない
        */
        if line[pos] == b' ' {
            sp_pos_1 = pos
        }
    }
    if sp_pos_1 == 0 {
        return Err(ParseError::SplitLineError(format!(
            "Passed line, expected to have 3 parts split by 2 whitespaces, doesn't have valid whitespace. Line content: {}",
            String::from_utf8_lossy(line)
        )))
    }
    for pos in sp_pos_1+1..eol-1 {
        if line[pos] == b' ' {
            sp_pos_2 = pos
        }
    }
    if sp_pos_2 == 0 {
        return Err(ParseError::SplitLineError(format!(
            "Passed line, expected to have 3 parts split by 2 whitespaces, doesn't have second valid white space. Line content: {}",
            String::from_utf8_lossy(line)
        )))
    }
    Ok((&line[0..sp_pos_1], &line[sp_pos_1+1..sp_pos_2], &line[sp_pos_2+1..eol]))
}
