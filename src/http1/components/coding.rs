pub(crate) enum ContentCoding {
    Gzip,
    Compress,
    Deflate,
    Identity
}
pub(crate) enum TransferCoding {
    Chunked,
}

pub(crate) struct Base64(/* &[u8] か */);
pub(crate) struct MD5(/* &[u8] か */);
pub(crate) struct Token68(/* &[u8] か */);