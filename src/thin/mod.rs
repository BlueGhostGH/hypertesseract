use std::ffi;

mod error;

#[derive(Debug)]
pub(crate) struct Tesseract
{
    base_api: *mut tesseract_sys::TessBaseAPI,
}

impl Tesseract
{
    pub(crate) fn set_variable(
        &mut self,
        name: &str,
        value: &str,
    ) -> Result<(), Error>
    {
        let c_name = ffi::CString::new(name)?;
        let c_value = ffi::CString::new(value)?;

        let ret = unsafe {
            tesseract_sys::TessBaseAPISetVariable(
                self.base_api,
                c_name.as_ptr(),
                c_value.as_ptr(),
            )
        };

        // `false` is returned if the name lookup fails
        if ret != 0 {
            Ok(())
        } else {
            Err(error::SetVariable::UnknownVariable { name: c_name })?
        }
    }
}

#[derive(Debug)]
pub(crate) enum Error
{
    Nul(ffi::NulError),

    SetVariable(error::SetVariable),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::Nul(nul_err) => write!(f, "{nul_err}"),

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
            Error::Nul(nul_err) => Some(nul_err),

            Error::SetVariable(set_variable_err) => Some(set_variable_err),
        }
    }
}

impl From<ffi::NulError> for Error
{
    fn from(nul_err: ffi::NulError) -> Self
    {
        Error::Nul(nul_err)
    }
}

impl From<error::SetVariable> for Error
{
    fn from(set_variable_err: error::SetVariable) -> Self
    {
        Error::SetVariable(set_variable_err)
    }
}
