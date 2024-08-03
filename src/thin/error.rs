use std::ffi;

#[derive(Debug)]
pub(crate) enum SetVariable
{
    UnknownVariable
    {
        name: ffi::CString
    },
}

impl ::std::fmt::Display for SetVariable
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Self::UnknownVariable { name } => {
                // SAFETY: This `&CString` was formed from a `&str`. If this
                // fails, that means the arguments provided to `set_variable`
                // broke the `&str` UTF-8 validity contract.
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
