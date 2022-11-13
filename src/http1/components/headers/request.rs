use crate::http1::components::{
    media_type::MediaType,
    qvalue::Qvalue,
    coding::{ContentCoding, Base64, MD5, Token68},
    language_tag::LanguageTag
};


pub(crate) struct Accept(
    Vec<(MediaType, Option<Qvalue>)>
);
pub(crate) struct AcceptEncoding(
    Vec<(ContentCoding, Option<Qvalue>)>
);
pub(crate) struct AcceptLanguage(
    Vec<(LanguageTag, Option<Qvalue>)>
);
pub(crate) enum Authorization {
    Basic(Base64),
    Bearer(Token68),
    Digest(MD5),
}