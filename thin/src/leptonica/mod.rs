use std::ptr;

mod error;

pub use error::Error;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Pix
{
    pub(crate) pix: ptr::NonNull<sys::leptonica::PIX>,
}

impl Pix
{
    pub fn create(width: i32, height: i32, depth: i32) -> Self
    {
        let pix = unsafe {
            let ptr = sys::leptonica::pixCreate(width, height, depth);

            // Let us pray allocation never ever fails
            ptr::NonNull::new_unchecked(ptr)
        };

        Pix { pix }
    }

    pub fn set_data(&mut self, data: &[u8]) -> Result<()>
    {
        let ret = unsafe {
            sys::leptonica::pixSetData(
                self.pix.as_ptr(),
                data.as_ptr().cast_mut().cast(),
            )
        };

        // `-1` is returned if initialisation fails
        // No further reason for failure is provided
        if ret != -1 {
            Ok(())
        } else {
            Err(error::set_data::Error::FailedToSetData)?
        }
    }
}

impl Drop for Pix
{
    fn drop(&mut self)
    {
        unsafe { sys::leptonica::pixDestroy(&mut self.pix.as_ptr()) }
    }
}
