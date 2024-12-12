#[derive(Debug)]
pub enum Error {
    DecodingEnumVariant(u8),
    SimpleDecodeError(String),
    SimpleDecodeTryFrom,
    SimpleEncodeError,
}
