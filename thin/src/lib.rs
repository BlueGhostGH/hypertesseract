use std::{ffi, ptr};

mod error;
pub mod variables;

#[derive(Debug)]
pub struct Tesseract
{
    base_api: *mut sys::TessBaseAPI,
}

impl Tesseract
{
    pub fn create() -> Self
    {
        Tesseract {
            base_api: unsafe { sys::TessBaseAPICreate() },
        }
    }

    pub fn init(
        &mut self,
        language: Option<&'static ffi::CStr>,
    ) -> Result<(), Error>
    {
        let ret = unsafe {
            sys::TessBaseAPIInit3(
                self.base_api,
                ptr::null(),
                language.map(ffi::CStr::as_ptr).unwrap_or_else(ptr::null),
            )
        };

        // `-1` is returned if initialisation fails
        // No further reason for failure is provided
        if ret != -1 {
            Ok(())
        } else {
            Err(error::init::Error::FailedToInit)?
        }
    }

    pub fn set_variable(
        &mut self,
        name: &'static ffi::CStr,
        value: &ffi::CStr,
    ) -> Result<(), Error>
    {
        let ret = unsafe {
            sys::TessBaseAPISetVariable(
                self.base_api,
                name.as_ptr(),
                value.as_ptr(),
            )
        };

        // `false` is returned if the name lookup fails
        if ret != 0 {
            Ok(())
        } else {
            Err(error::set_variable::Error::UnknownVariable { name })?
        }
    }

    pub fn set_page_seg_mode(&mut self, mode: sys::TessPageSegMode)
    {
        unsafe { sys::TessBaseAPISetPageSegMode(self.base_api, mode) }
    }
}

#[derive(Debug)]
pub enum Error
{
    Init(error::init::Error),
    SetVariable(error::set_variable::Error),
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
        }
    }
}

impl From<error::init::Error> for Error
{
    fn from(init_err: error::init::Error) -> Self
    {
        Error::Init(init_err)
    }
}

impl From<error::set_variable::Error> for Error
{
    fn from(set_variable_err: error::set_variable::Error) -> Self
    {
        Error::SetVariable(set_variable_err)
    }
}
