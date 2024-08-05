mod builder;
mod thin;

pub use builder::language::Language;
pub use builder::page_seg_mode::PageSegMode;

#[derive(Debug)]
pub struct Tesseract
{
    _base_api: thin::Tesseract,
}

impl Tesseract
{
    pub fn builder() -> builder::Builder<((), (), (), ())>
    {
        builder::Builder::default()
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error
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

#[cfg(test)]
mod tests
{
    // For some reason you need OpenSSL for tesseract to link properly on Windows...
    #[cfg(target_os = "windows")]
    #[allow(unused_imports)]
    use openssl;

    #[test]
    fn builder_compiles()
    {
        let _t = crate::Tesseract::builder()
            .assume_numeric_input()
            .whitelist_str("abcdef")
            .unwrap()
            .language(crate::Language::English)
            .page_seg_mode(crate::PageSegMode::SingleLine)
            .build()
            .unwrap();

        assert!(true);
    }
}
