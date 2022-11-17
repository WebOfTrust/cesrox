use std;
use std::fmt::{self, Display, Formatter};

use serde::{de, ser};

use crate::derivation::basic::Basic;
use crate::error::serializer_error::Error::{Eof, ExpectedArray, ExpectedArrayComma, ExpectedArrayEnd, ExpectedBoolean, ExpectedEnum, ExpectedInteger, ExpectedMap, ExpectedMapColon, ExpectedMapComma, ExpectedMapEnd, ExpectedNull, ExpectedString, Message, Syntax, TrailingCharacters};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Message(msg) => formatter.write_str(msg),
            Eof => formatter.write_str("unexpected end of input"),
            Syntax => formatter.write_str("incorrect syntax"),
            ExpectedBoolean => formatter.write_str("incorrect input: expected boolean"),
            ExpectedInteger => formatter.write_str("incorrect input: expected integer"),
            ExpectedString => formatter.write_str("incorrect input: expected string"),
            ExpectedNull => formatter.write_str("incorrect input: expected null"),
            ExpectedArray => formatter.write_str("incorrect input: expected array"),
            ExpectedArrayComma => {
                formatter.write_str("incorrect input: expected array comma")
            }
            ExpectedArrayEnd => formatter.write_str("incorrect input: expected array end"),
            ExpectedMap => formatter.write_str("incorrect input: expected map"),
            ExpectedMapColon => formatter.write_str("incorrect input: expected map colon"),
            ExpectedMapComma => formatter.write_str("incorrect input: expected map comma"),
            ExpectedMapEnd => formatter.write_str("incorrect input: expected map end"),
            ExpectedEnum => formatter.write_str("incorrect input: expected enum"),
            TrailingCharacters => {
                formatter.write_str("incorrect input: unexpected trailing characters")
            }
        }
    }
}

#[test]
fn test_from_str() {
    assert_eq!(&format!("{}", Message("foo".to_string())), "foo");
    assert_eq!(&format!("{}", Eof), "unexpected end of input");
    assert_eq!(&format!("{}", Syntax), "incorrect syntax");
    assert_eq!(&format!("{}", ExpectedBoolean), "incorrect input: expected boolean");
    assert_eq!(&format!("{}", ExpectedInteger), "incorrect input: expected integer");
    assert_eq!(&format!("{}", ExpectedString), "incorrect input: expected string");
    assert_eq!(&format!("{}", ExpectedNull), "incorrect input: expected null");
    assert_eq!(&format!("{}", ExpectedArray), "incorrect input: expected array");
    assert_eq!(&format!("{}", ExpectedArrayComma), "incorrect input: expected array comma");
    assert_eq!(&format!("{}", ExpectedArrayEnd), "incorrect input: expected array end");
    assert_eq!(&format!("{}", ExpectedMap), "incorrect input: expected map");
    assert_eq!(&format!("{}", ExpectedMapColon), "incorrect input: expected map colon");
    assert_eq!(&format!("{}", ExpectedMapComma), "incorrect input: expected map comma");
    assert_eq!(&format!("{}", ExpectedMapEnd), "incorrect input: expected map end");
    assert_eq!(&format!("{}", ExpectedEnum), "incorrect input: expected enum");
    assert_eq!(&format!("{}", TrailingCharacters), "incorrect input: unexpected trailing characters");
}

impl std::error::Error for Error {}
