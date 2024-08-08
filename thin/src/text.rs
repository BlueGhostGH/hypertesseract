use std::{ffi, ptr};

#[derive(Debug)]
pub struct Text
{
    ptr: ptr::NonNull<i8>,
}

impl Text
{
    pub(super) unsafe fn new_unchecked(ptr: *mut i8) -> Self
    {
        Text {
            ptr: ptr::NonNull::new_unchecked(ptr),
        }
    }
}

impl AsRef<ffi::CStr> for Text
{
    fn as_ref(&self) -> &ffi::CStr
    {
        unsafe { ffi::CStr::from_ptr(self.ptr.as_ptr()) }
    }
}

impl Drop for Text
{
    fn drop(&mut self)
    {
        unsafe { sys::TessDeleteText(self.ptr.as_ptr()) }
    }
}
