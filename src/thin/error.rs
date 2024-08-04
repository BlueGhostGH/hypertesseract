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
