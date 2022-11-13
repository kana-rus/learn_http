use crate::http1::components::{
    entity_tag::EntityTag,
    uri::URI,
    date::Date
};
use super::FieldName;


pub(crate) enum AcceptRanges {
    Bytes, // 'bytes'; I accept byte range request
    None,  // 'none' ; I don't accept any range request
}
pub(crate) struct Age(
    u32
);
pub(crate) struct ETag(
    EntityTag
);
pub(crate) struct Location(
    URI
);
// pub(crate) struct ProxyAuthenticate
// what's "challenge" ?
pub(crate) enum RetryAfter {
    Date(Date),
    DeltaSeconds(usize),
}
pub(crate) struct Server(
    Vec<String>
);
pub(crate) struct Vary(
    Vec<FieldName>
);
// pub(crate) struct WWWAuthenticate
// what's "challenge" ?