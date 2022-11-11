pub(super) enum ParseError {
    ReadLineError(String),
    SplitLineError(String),
    InvalidUri(String),
    InvalidVersion(String),
}