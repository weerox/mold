#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NoOpeningTag,
    NoClosingTag,
    InvalidClosingTag,
}
