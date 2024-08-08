use std::{num, str};

#[derive(Debug)]
pub enum Error
{
    TryFromInt(num::TryFromIntError),
    Utf8(str::Utf8Error),

    Thin(thin::Error),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::TryFromInt(try_from_int_err) => {
                write!(f, "{try_from_int_err}")
            }
            Error::Utf8(utf8_err) => {
                write!(f, "{utf8_err}")
            }

            Error::Thin(thin_err) => write!(f, "{thin_err}"),
        }
    }
}

impl ::std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)>
    {
        match self {
            Error::TryFromInt(try_from_int_err) => Some(try_from_int_err),
            Error::Utf8(utf8_err) => Some(utf8_err),

            Error::Thin(thin_err) => Some(thin_err),
        }
    }
}

impl From<num::TryFromIntError> for Error
{
    fn from(try_from_int_err: num::TryFromIntError) -> Self
    {
        Error::TryFromInt(try_from_int_err)
    }
}

impl From<str::Utf8Error> for Error
{
    fn from(utf8_err: str::Utf8Error) -> Self
    {
        Error::Utf8(utf8_err)
    }
}

impl From<thin::Error> for Error
{
    fn from(thin_err: thin::Error) -> Self
    {
        Error::Thin(thin_err)
    }
}

impl From<thin::leptonica::Error> for Error
{
    fn from(leptonica_err: thin::leptonica::Error) -> Self
    {
        Error::Thin(leptonica_err.into())
    }
}
