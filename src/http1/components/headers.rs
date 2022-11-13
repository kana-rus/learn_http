mod general;
mod request;
mod response;
mod entity;


pub(crate) enum RequestHeader {
    Accept(request::Accept),
    // AcceptCharset,
    AcceptEncoding(request::AcceptEncoding),
    AcceptLanguage(request::AcceptLanguage),
    Authorization(request::Authorization),
    Expect(request::Expect),
    From(request::From),
    Host(request::Host),
    IfMatch(request::IfMatch),
    IfModifiedSince(request::IfModifiedSince),
    IfNoneMatch(request::IfNoneMatch),
    IfRange(request::IfRange),
    IfUnmodifiedSince(request::IfUnmodifiedSince),
    MaxForwords(request::MaxForwords),
    ProxyAuthorization(request::ProxyAuthorization),
    Range(request::Range),
    Referer(request::Referer),
    TE(request::TE),
    UserAgent(request::UserAgent),
}
pub(crate) enum ResponseHeader {
    AcceptRanges(response::AcceptRanges),
    Age(response::Age),
    ETag(response::ETag),
    Location(response::Location),
    // ProxyAuthenticate
    RetryAfter(response::RetryAfter),
    Server(response::Server),
    Vary(response::Vary),
    // WWWAuthenticate
}
pub(crate) enum GeneralHeader {
    CacheControl(general::CacheControl),
    Connection(general::Connection),
    Date(general::Date),
    // Pragma
    Trailer(general::Trailer),
    TransferEncoding(general::TransferEncoding),
    // Upgrade
    Via(general::Via),
    Warning(general::Warning),
}

enum FieldName {
    // request
    Accept,
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
    TE,
    UserAgent,

    // response
    AcceptRanges,
    Age,
    ETag,
    Location,
    RetryAfter,
    Server,
    Vary,

    // general
    CacheControl,
    Connection,
    Date,
    Trailer,
    TransferEncoding,
    Via,
    Warning,
}