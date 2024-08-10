use std::{ffi, ptr};

pub mod leptonica;

mod error;
mod text;
pub mod variables;

pub use error::Error;
type Result<T> = ::std::result::Result<T, Error>;

pub use text::Text;

#[derive(Debug)]
pub struct Tesseract
{
    base_api: ptr::NonNull<sys::TessBaseAPI>,
}

impl Tesseract
{
    pub fn create() -> Self
    {
        let base_api = unsafe {
            let ptr = sys::TessBaseAPICreate();

            // Let us pray allocation never ever fails
            ptr::NonNull::new_unchecked(ptr)
        };

        Tesseract { base_api }
    }

    pub fn init(&mut self, language: Option<&'static ffi::CStr>) -> Result<()>
    {
        let ret = unsafe {
            sys::TessBaseAPIInit3(
                self.base_api.as_ptr(),
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
                self.base_api.as_ptr(),
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
        unsafe { sys::TessBaseAPISetPageSegMode(self.base_api.as_ptr(), mode) }
    }

    pub fn set_image(&mut self, pix: &mut leptonica::Pix)
    {
        unsafe {
            sys::TessBaseAPISetImage2(self.base_api.as_ptr(), pix.pix.as_ptr())
        }
    }

    pub fn recognize(&mut self) -> Result<()>
    {
        let ret = unsafe {
            sys::TessBaseAPIRecognize(self.base_api.as_ptr(), ptr::null_mut())
        };

        // `0` is returned if recognition succeeds
        // No further reason for failure is provided
        if ret == 0 {
            Ok(())
        } else {
            Err(error::recognize::Error::FailedToRecognize)?
        }
    }

    pub fn mean_text_conf(&mut self) -> i32
    {
        unsafe { sys::TessBaseAPIMeanTextConf(self.base_api.as_ptr()) }
    }

    pub fn get_utf8_text(&mut self) -> Result<Text>
    {
        let text =
            unsafe { sys::TessBaseAPIGetUTF8Text(self.base_api.as_ptr()) };

        // `nullptr` can be returned if something goes wrong
        if !text.is_null() {
            Ok(unsafe { Text::new_unchecked(text) })
        } else {
            Err(error::get_utf8_text::Error::FailedToGetUtf8Text)?
        }
    }
}

impl Drop for Tesseract
{
    fn drop(&mut self)
    {
        unsafe {
            sys::TessBaseAPIDelete(self.base_api.as_ptr());
        }
    }
}
