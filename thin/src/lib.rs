use std::{ffi, ptr};

pub mod leptonica;

mod error;
pub mod variables;

pub use error::Error;
type Result<T> = ::std::result::Result<T, Error>;

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

    pub fn init(&mut self, language: Option<&'static ffi::CStr>) -> Result<()>
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
    ) -> Result<()>
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
