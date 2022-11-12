pub(crate) enum ParseError {
    ReadLineError(String),
    SplitLineError(String),
    InvalidUri(String),
    InvalidQueryParameter(String),
    InvalidVersion(String),
    UnknownHeader(String),
}