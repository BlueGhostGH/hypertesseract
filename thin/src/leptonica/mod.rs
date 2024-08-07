use std::num;

mod error;

pub use error::Error;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Pix
{
    pix: *mut sys::leptonica::PIX,
}

impl Pix
{
    pub fn create(width: i32, height: i32, depth: i32) -> Self
    {
        Pix {
            pix: unsafe { sys::leptonica::pixCreate(width, height, depth) },
        }
    }

    pub fn set_data(&mut self, data: &[u8]) -> Result<()>
    {
        let ret = unsafe {
            sys::leptonica::pixSetData(
                self.pix,
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
