mod symbols;
mod server;
mod utils;

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
                CacheControl(String),
                Connection(String),
                Date(String),
                Pragma(String),
                Trailer(String),
                TransferEncoding(String),
                Upgrade(String),
                Via(String),
                Warning(String),
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
        method:       Method,
        request_uri:  RequestUri,
        http_version: HttpVersion,
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
            Authority(String),
        }
    enum HeaderInRequest {
        GeneralHeader(GeneralHeader),
        RequestHeader(RequestHeader),
        EntityHeader(EntityHeader),
    }
        enum RequestHeader {
            Accept(String),
            AcceptCharset(String),
            AcceptEncoding(String),
            AcceptLanguage(String),
            Authorization(String),
            Expect(String),
            From(String),
            Host(String),
            IfMatch(String),
            IfModifiedSince(String),
            IfNoneMatch(String),
            IfRange(String),
            IfUnmodifiedSince(String),
            MaxForwords(String),
            ProxyAuthorization(String),
            Range(String),
            Referer(String),
            Te(String),
            UserAgent(String),
        }

pub(super) struct Response {
    status_line: StatusLine,
    headers: Vec<HeaderInResponse>,
    message_body: MessageBody,
}
    struct StatusLine {
        http_version: HttpVersion,
        status_code: StatusCode,
        reason_phrase: Text,
    }
        enum StatusCode {
            Continue                     = 100,
            SwithingProtocols            = 101,
            Ok                           = 200,
            Created                      = 201,
            Accepted                     = 202,
            NonAuthoritativeInformation  = 203,
            NoContent                    = 204,
            ResetContent                 = 205,
            PartialContent               = 206,
            MultipleChoices              = 300,
            MovedPermanently             = 301,
            Found                        = 302,
            SeeOther                     = 303,
            NotModified                  = 304,
            UseProxy                     = 305,
            TemporaryRedirect            = 307,
            BadRequest                   = 400,
            Unauthorized                 = 401,
            PaymentRequired              = 402,
            Forbidden                    = 403,
            NotFound                     = 404,
            MethodNotAllowed             = 405,
            NotAcceptable                = 406,
            ProxyAuthenticationRequired  = 407,
            RequestTimeOut               = 408,
            Conflict                     = 409,
            Gone                         = 410,
            LengthRequired               = 411,
            PreconditionFailed           = 412,
            RequestEntityTooLarge        = 413,
            RequestUriTooLarge           = 414,
            UnsupportedMediaType         = 415,
            RequestedRangeNotSatisfiable = 416,
            ExpectationFailed            = 417,
            InternalServerError          = 500,
            NotImplemented               = 501,
            BadGateway                   = 502,
            ServiceUnavalable            = 503,
            GatewayTimeout               = 504,
            HttpVersionNotSupported      = 505,
        }
    enum HeaderInResponse {
        GeneralHeader(GeneralHeader),
        ResponseHeader(ResponseHeader),
        EntityHeader(EntityHeader),
    }
        enum ResponseHeader {
            AcceptRanges,
            Age,
            Etag,
            LOcation,
            ProxyAuthenticate,
            RetryAfter,
            Server,
            Vary,
            WwwAuthenticate,
        }
enum EntityHeader {
    Allow(String),
    ContentEncoding(String),
    ContentLanguage(String),
    ContentLength(String),
    ContentLocation(String),
    ContentMd5(String),
    ContentRange(String),
    ContentType(String),
    Expires(String),
    LastModified(String),
}
enum HttpVersion {
    First,
    // Second,
    // Third,
}