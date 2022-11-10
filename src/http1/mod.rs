mod symbols;

use self::symbols::{Token, Lws, Text, Separators};


pub(super) struct Message {
    start_line:   StartLine,
    headers:      Vec<MessageHeader>,
    message_body: Option<MessageBody>,
}
    enum StartLine {
        RequestLine(RequestLine),
        StatusLine(StatusLine),
    }
    struct MessageHeader {
        field_name:  FieldName,
        field_value: Vec<FieldValue>,
    }
        enum FieldName {
            GeneralHeader(GeneralHeader),
            Other(Token),
        }
            enum GeneralHeader {
                CacheControl,
                Connection,
                Date,
                Pragma,
                Trailer,
                TransferEncoding,
                Upgrade,
                Via,
                Warning,
            }
        enum FieldValue {
            FieldContent(FieldContent),
            Lws(Lws),
        }
            enum FieldContent {
                Text(Text),
                Token(Token),
                Separators(Separators),
                QuotedString(Text),
            }
    enum MessageBody {
        EntityBody(String),
        TransferEncodedEntityBody(String),
    }

pub(super) struct Request {
    request_line: RequestLine,
    headers: Vec<HeaderInRequest>,
    message_body: MessageBody,
}
    struct RequestLine {
        method:      Method,
        request_uri: RequestUri,
    }
        enum Method {
            Options,
            Get,
            Head,
            Post,
            Put,
            Delete,
            Trace,
            Connect,
            Extension(Token),
        }
        enum RequestUri {
            Asterisk,
            AbsoluteUri(String),
            AbsPath(String),
            Authority,
        }
    enum HeaderInRequest {
        GeneralHeader(GeneralHeader),
        RequestHeader(RequestHeader),
        EntityHeader(EntityHeader),
    }
        enum RequestHeader {
            Accept,
            AcceptCharset,
            AcceptEncoding,
            AcceptLanguage,
            Authorization,
            Expect,
            From,
            Host,
            IfMatch,
            IfModifiedSince,
            IfNoneMatch,
            IfRange,
            IfUnmodifiedSince,
            MaxForwords,
            ProxyAuthorization,
            Range,
            Referer,
            Te,
            UserAgent,
        }

pub(super) struct Response {
    status_line: StatusLine,
    headers: Vec<HeaderInResponse>,
    message_body: MessageBody,
}
    struct StatusLine {
        
    }