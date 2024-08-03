use crate::thin;

// Tuple goes as follows:
// [0] - assume_numeric
#[derive(Debug)]
struct Builder<Fields = ((),)>
{
    fields: Fields,
}

fn build(assume_numeric_input: bool) -> Result<crate::Tesseract, thin::Error>
{
    let mut thin_tess = crate::thin::Tesseract::create();

    thin_tess.set_variable(
        thin::variables::ASSUME_NUMERIC_INPUT,
        if assume_numeric_input {
            constants::TRUE
        } else {
            constants::FALSE
        },
    )?;

    Ok(crate::Tesseract {
        base_api: thin_tess,
    })
}

impl Builder<((),)>
{
    fn assume_numeric_input(self) -> Builder<(bool,)>
    {
        let Builder { fields: ((),) } = self;
        Builder { fields: (true,) }
    }

    fn build(self) -> Result<crate::Tesseract, thin::Error>
    {
        build(false)
    }
}

mod constants
{
    use std::ffi;

    pub(super) const TRUE: &'static ffi::CStr = unsafe {
        // NOTE: Arbitrary choice out of the valid values.
        let bytes = b"1\0";

        ffi::CStr::from_bytes_with_nul_unchecked(bytes)
    };

    pub(super) const FALSE: &'static ffi::CStr = unsafe {
        // NOTE: Arbitrary choice out of the valid values.
        let bytes = b"0\0";

        ffi::CStr::from_bytes_with_nul_unchecked(bytes)
    };
}
