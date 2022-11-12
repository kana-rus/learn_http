use crate::http1::{Method, RequestUri, MessageBody, HeaderInRequest, utils::consts::BUFF_SIZE};
use super::utils::errors::ParseError;


struct Request {
    method:  Method,
    uri:     RequestUri,
    headers: Vec<HeaderInRequest>,
    body:    Option<MessageBody>,
}

fn parse_request(buf: &[u8]) -> Result<Request, ParseError> {
    let mut buf_read_pos = 0;

    let Some(request_line) = read_line(buf, &mut buf_read_pos) else {
        return Err(ParseError::ReadLineError(format!(
            "Empty request"
        )))
    };
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

    let mut headers = Vec::new();
    while let Some(next_line) = read_line(buf, &mut buf_read_pos) {
        if next_line.is_empty() {break}
        headers.push(
            parse_header(next_line)?
        )
    }

    let mut body = None;
    if let Some(line) = read_line(buf, &mut buf_read_pos) {
        body = Some(MessageBody::EntityBody(String::from_utf8_lossy(line).into()))
    }
    
    Ok(Request { method, uri, headers, body })

}

fn read_line<'b>(buf: &'b [u8], current_pos: &mut usize) -> Option<&'b [u8]> {
    if *current_pos >= BUFF_SIZE {return None}
    for eol in *current_pos..BUFF_SIZE-1 {
        if buf[eol]   == b'\r'
        && buf[eol+1] == b'\n' {
            let line = &buf[*current_pos..eol];
            *current_pos = eol + 2;
            return Some(line)
        }
    }
    None
}
fn split_sp_2(line: &[u8]) -> Result<(&[u8], &[u8], &[u8]), ParseError> {
    let eol = line.len();
    let (mut sp_pos_1, mut sp_pos_2) = (0, 0);
    
    for pos in 1..eol-3 {
        /*
            o o o o ...  o   o   o  eol
            0 1 2 3 ... -3  -2  -1  len
                         |--> これ以降に sp_pos_1 があると 3 parts になりようがない
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
fn parse_header(buf: &[u8]) -> Result<HeaderInRequest, ParseError> {
    use {
        HeaderInRequest::*, 
        crate::http1::{RequestHeader::*, GeneralHeader::*, EntityHeader::*},
    };
    let eol = buf.len();
    let mut colon_pos = eol;

    for pos in 0..eol {
        if buf[pos] == b':' {
            colon_pos = pos
        }
    }
    if colon_pos == 0 {
        return Err(ParseError::InvalidUri(format!(
            "Invalid header line: {}",
            String::from_utf8_lossy(buf)
        )))
    }

    let (field, value) = (
        &buf[0..colon_pos],
        &buf[colon_pos+1..eol]
    );
    match field {
        b"Accept"             => Ok(RequestHeader(Accept(String::from_utf8_lossy(value).into()))),
        b"Accept-Charset"     => Ok(RequestHeader(AcceptCharset(String::from_utf8_lossy(value).into()))),
        b"Accept-Encoding"    => Ok(RequestHeader(AcceptEncoding(String::from_utf8_lossy(value).into()))),
        b"Accept-Language"    => Ok(RequestHeader(AcceptLanguage(String::from_utf8_lossy(value).into()))),
        b"Authorization"      => Ok(RequestHeader(Authorization(String::from_utf8_lossy(value).into()))),
        b"Expect"             => Ok(RequestHeader(Expect(String::from_utf8_lossy(value).into()))),
        b"From"               => Ok(RequestHeader(From(String::from_utf8_lossy(value).into()))),
        b"Host"               => Ok(RequestHeader(Host(String::from_utf8_lossy(value).into()))),
        b"If-Match"           => Ok(RequestHeader(IfMatch(String::from_utf8_lossy(value).into()))),
        b"If-Modified-Since"  => Ok(RequestHeader(IfModifiedSince(String::from_utf8_lossy(value).into()))),
        b"If-None_Match"      => Ok(RequestHeader(IfNoneMatch(String::from_utf8_lossy(value).into()))),
        b"If-Range"           => Ok(RequestHeader(IfRange(String::from_utf8_lossy(value).into()))),
        b"If-UnmodifiedSince" => Ok(RequestHeader(IfUnmodifiedSince(String::from_utf8_lossy(value).into()))),
        b"Max-Forwords"       => Ok(RequestHeader(MaxForwords(String::from_utf8_lossy(value).into()))),
        b"Proxy-Authenticate" => Ok(RequestHeader(ProxyAuthorization(String::from_utf8_lossy(value).into()))),
        b"Range"              => Ok(RequestHeader(Range(String::from_utf8_lossy(value).into()))),
        b"Referer"            => Ok(RequestHeader(Referer(String::from_utf8_lossy(value).into()))),
        b"TE"                 => Ok(RequestHeader(Te(String::from_utf8_lossy(value).into()))),
        b"UserAgent"          => Ok(RequestHeader(UserAgent(String::from_utf8_lossy(value).into()))),

        b"Cache-Control"      => Ok(GeneralHeader(CacheControl(String::from_utf8_lossy(field).into()))),
        b"Connection"         => Ok(GeneralHeader(Connection(String::from_utf8_lossy(field).into()))),
        b"Date"               => Ok(GeneralHeader(Date(String::from_utf8_lossy(field).into()))),
        b"Pragma"             => Ok(GeneralHeader(Pragma(String::from_utf8_lossy(field).into()))),
        b"Trailer"            => Ok(GeneralHeader(Trailer(String::from_utf8_lossy(field).into()))),
        b"Transfer-Encoding"  => Ok(GeneralHeader(TransferEncoding(String::from_utf8_lossy(field).into()))),
        b"Upgrade"            => Ok(GeneralHeader(Upgrade(String::from_utf8_lossy(field).into()))),
        b"Via"                => Ok(GeneralHeader(Via(String::from_utf8_lossy(field).into()))),
        b"Warning"            => Ok(GeneralHeader(Warning(String::from_utf8_lossy(field).into()))),

        b"Allow"              => Ok(EntityHeader(Allow(String::from_utf8_lossy(field).into()))),
        b"Content-Encoding"   => Ok(EntityHeader(ContentEncoding(String::from_utf8_lossy(field).into()))),
        b"Content-Language"   => Ok(EntityHeader(ContentLanguage(String::from_utf8_lossy(field).into()))),
        b"Content-Length"     => Ok(EntityHeader(ContentLength(String::from_utf8_lossy(field).into()))),
        b"Content-Location"   => Ok(EntityHeader(ContentLocation(String::from_utf8_lossy(field).into()))),
        b"Content-MD5"        => Ok(EntityHeader(ContentMd5(String::from_utf8_lossy(field).into()))),
        b"Content-Range"      => Ok(EntityHeader(ContentRange(String::from_utf8_lossy(field).into()))),
        b"Content-Type"       => Ok(EntityHeader(ContentType(String::from_utf8_lossy(field).into()))),
        b"Expires"            => Ok(EntityHeader(Expires(String::from_utf8_lossy(field).into()))),
        b"Last-Modified"      => Ok(EntityHeader(LastModified(String::from_utf8_lossy(field).into()))),

        _ => Err(ParseError::UnknownHeader(String::from_utf8_lossy(field).into())),
    }
}
