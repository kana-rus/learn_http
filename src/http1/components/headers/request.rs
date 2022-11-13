use std::ops::RangeInclusive;
use crate::http1::components::{
    media_type::MediaType,
    qvalue::Qvalue,
    coding::ContentCoding,
    language_tag::LanguageTag,
    email::Email,
    entity_tag::EntityTag,
    date::Date,
    credentials::Credentials,
    uri::URI,
};


pub(crate) struct Accept(
    Vec<(MediaType, Option<Qvalue>)>
);
// pub(crate) struct AcceptCharset
// what's the list of charset ?
pub(crate) struct AcceptEncoding(
    Vec<(ContentCoding, Option<Qvalue>)>
);
pub(crate) struct AcceptLanguage(
    Vec<(LanguageTag, Option<Qvalue>)>
);
pub(crate) struct Authorization(
    Credentials
);
pub(crate) enum Expect {
    Continue,
}
pub(crate) struct From(
    Email
);
pub(crate) struct Host {
    host: String,
    port: Option<u16>,
}
pub(crate) struct IfMatch(
    Vec<EntityTag>
);
pub(crate) struct IfModifiedSince(
    Date
);
pub(crate) struct IfNoneMatch(
    Vec<EntityTag>
);
pub(crate) enum IfRange {
    EntityTag(EntityTag),
    Date(Date),
}
pub(crate) struct IfUnmodifiedSince(
    Date
);
pub(crate) struct MaxForwords(
    u8
);
pub(crate) struct ProxyAuthorization(
    Credentials
);
pub(crate) struct Range(
    Vec<RangeInclusive<usize>>
);
pub(crate) struct Referer(
    URI // or relative URI, but relative URI can
    // imediately converted to (absolute) URI
    // just when parsing given TCP stream
);
pub(crate) struct TE(
    Vec<TransferCodingAcception>
);
    enum TransferCodingAcception {
        Trailer,
        Extension(ContentCoding, Qvalue),
    }
pub(crate) struct UserAgent(
    Vec<String>
);