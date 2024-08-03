use std::ffi;

#[derive(Debug)]
pub(crate) enum SetVariable
{
    UnknownVariable
    {
        name: &'static ffi::CStr
    },
}

impl ::std::fmt::Display for SetVariable
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

impl ::std::error::Error for SetVariable
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        match self {
            Self::UnknownVariable { .. } => None,
        }
    }
}
