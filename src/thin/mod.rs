use std::ffi;

mod error;
pub(crate) mod variables;

#[derive(Debug)]
pub(crate) struct Tesseract
{
    base_api: *mut tesseract_sys::TessBaseAPI,
}

impl Tesseract
{
    pub(crate) fn create() -> Self
    {
        Tesseract {
            base_api: unsafe { tesseract_sys::TessBaseAPICreate() },
        }
    }

    pub(crate) fn set_variable(
        &mut self,
        name: &'static ffi::CStr,
        value: &ffi::CStr,
    ) -> Result<(), Error>
    {
        let ret = unsafe {
            tesseract_sys::TessBaseAPISetVariable(
                self.base_api,
                name.as_ptr(),
                value.as_ptr(),
            )
        };

        // `false` is returned if the name lookup fails
        if ret != 0 {
            Ok(())
        } else {
            Err(error::SetVariable::UnknownVariable { name })?
        }
    }
}

#[derive(Debug)]
pub(crate) enum Error
{
    SetVariable(error::SetVariable),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::SetVariable(set_variable_err) => {
                write!(
                    f,
                    "error while calling set_variable: {set_variable_err}"
                )
            }
        }
    }
}

impl ::std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)>
    {
        match self {
            Error::SetVariable(set_variable_err) => Some(set_variable_err),
        }
    }
}

impl From<error::SetVariable> for Error
{
    fn from(set_variable_err: error::SetVariable) -> Self
    {
        Error::SetVariable(set_variable_err)
    }
}
