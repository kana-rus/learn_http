use super::coding::{Base64, Token68, MD5};

pub(crate) enum Credentials {
    Basic(Base64),
    Bearer(Token68),
    Digest(MD5),
}