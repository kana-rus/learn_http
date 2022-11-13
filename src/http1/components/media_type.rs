pub(crate) enum MediaType {
    Text(TextSubtype),
    Image(ImageSubtype),
    Audio(AudioSubtype),
    Application(ApplicationSubtype),
}
pub enum TextSubtype {
    HTML,
    PLAIN,
}
pub enum ImageSubtype {
    JPEG,
    PNG,
    WEBP,
}
pub enum AudioSubtype {
    Basic,
}
pub enum ApplicationSubtype {
    JSON,
}