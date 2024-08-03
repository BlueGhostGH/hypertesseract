mod builder;
mod thin;

#[derive(Debug)]
pub struct Tesseract
{
    base_api: thin::Tesseract,
}

impl Tesseract {}

#[derive(Debug)]
pub(crate) enum Error
{
    Thin(thin::Error),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::Thin(thin_err) => write!(f, "{thin_err}"),
        }
    }
}

impl ::std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)>
    {
        match self {
            Error::Thin(thin_err) => Some(thin_err),
        }
    }
}

impl From<thin::Error> for Error
{
    fn from(thin_err: thin::Error) -> Self
    {
        Error::Thin(thin_err)
    }
}
