use crate::leptonica;

#[derive(Debug)]
pub enum Error
{
    Init(init::Error),
    SetVariable(set_variable::Error),
    Recognize(recognize::Error),
    GetUtf8Text(get_utf8_text::Error),

    Leptonica(leptonica::Error),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::Init(init_err) => {
                write!(f, "error while calling init: {init_err}")
            }
            Error::SetVariable(set_variable_err) => {
                write!(
                    f,
                    "error while calling set_variable: {set_variable_err}"
                )
            }
            Error::Recognize(recognize_err) => {
                write!(f, "error while calling recognize: {recognize_err}")
            }
            Error::GetUtf8Text(get_utf8_text_err) => {
                write!(
                    f,
                    "error while calling get_utf8_text: {get_utf8_text_err}"
                )
            }

            Error::Leptonica(leptonica_err) => {
                write!(f, "{leptonica_err}")
            }
        }
    }
}

impl ::std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)>
    {
        match self {
            Error::Init(init_err) => Some(init_err),
            Error::SetVariable(set_variable_err) => Some(set_variable_err),
            Error::Recognize(recognize_err) => Some(recognize_err),
            Error::GetUtf8Text(get_utf8_text_err) => Some(get_utf8_text_err),

            Error::Leptonica(leptonica_err) => Some(leptonica_err),
        }
    }
}

impl From<init::Error> for Error
{
    fn from(init_err: init::Error) -> Self
    {
        Error::Init(init_err)
    }
}

impl From<set_variable::Error> for Error
{
    fn from(set_variable_err: set_variable::Error) -> Self
    {
        Error::SetVariable(set_variable_err)
    }
}

impl From<recognize::Error> for Error
{
    fn from(recognize_err: recognize::Error) -> Self
    {
        Error::Recognize(recognize_err)
    }
}

impl From<get_utf8_text::Error> for Error
{
    fn from(get_utf8_text_err: get_utf8_text::Error) -> Self
    {
        Error::GetUtf8Text(get_utf8_text_err)
    }
}

impl From<leptonica::Error> for Error
{
    fn from(leptonica_err: leptonica::Error) -> Self
    {
        Error::Leptonica(leptonica_err)
    }
}

pub(super) mod init
{
    #[derive(Debug)]
    pub enum Error
    {
        // TODO: Make more in depth/detailed
        FailedToInit,
    }

    impl ::std::fmt::Display for Error
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
        {
            match self {
                Self::FailedToInit => {
                    write!(f, "failed to initialise tesseract")
                }
            }
        }
    }

    impl ::std::error::Error for Error
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
        {
            match self {
                Self::FailedToInit => None,
            }
        }
    }
}

pub(super) mod set_variable
{
    use std::ffi;

    #[derive(Debug)]
    pub enum Error
    {
        UnknownVariable
        {
            name: &'static ffi::CStr
        },
    }

    impl ::std::fmt::Display for Error
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
        {
            match self {
                Self::UnknownVariable { name } => {
                    // SAFETY: This `&CStr` was formed from a valid `&'static str`.
                    let name = unsafe { name.to_str().unwrap_unchecked() };

                    write!(f, "unknown variable \"{name}\"")
                }
            }
        }
    }

    impl ::std::error::Error for Error
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
        {
            match self {
                Self::UnknownVariable { .. } => None,
            }
        }
    }
}

pub(super) mod recognize
{
    #[derive(Debug)]
    pub enum Error
    {
        // TODO: Make more in depth/detailed
        FailedToRecognize,
    }

    impl ::std::fmt::Display for Error
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
        {
            match self {
                Self::FailedToRecognize => {
                    write!(f, "failed to recognize text")
                }
            }
        }
    }

    impl ::std::error::Error for Error
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
        {
            match self {
                Self::FailedToRecognize => None,
            }
        }
    }
}

pub(super) mod get_utf8_text
{
    #[derive(Debug)]
    pub enum Error
    {
        // TODO: Make more in depth/detailed
        FailedToGetUtf8Text,
    }

    impl ::std::fmt::Display for Error
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
        {
            match self {
                Self::FailedToGetUtf8Text => {
                    write!(f, "failed to get UTF-8 text")
                }
            }
        }
    }

    impl ::std::error::Error for Error
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
        {
            match self {
                Self::FailedToGetUtf8Text => None,
            }
        }
    }
}
