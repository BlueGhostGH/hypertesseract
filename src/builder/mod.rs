use std::{borrow, ffi};

use crate::thin;

// NOTE: Gotta hide the combinatorial explosion..
mod explosion;

// Tuple goes as follows:
// [0] - assume_numeric_input
// [1] - whitelist
#[derive(Debug)]
struct Builder<Fields = ((), ())>
{
    fields: Fields,
}

fn build(
    assume_numeric_input: bool,
    whitelist: Option<borrow::Cow<'_, ffi::CStr>>, // TODO: Maybe figure something better
) -> Result<crate::Tesseract, crate::Error>
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

    if let Some(whitelist) = whitelist {
        thin_tess
            .set_variable(thin::variables::ASSUME_NUMERIC_INPUT, &whitelist)?
    }

    Ok(crate::Tesseract {
        base_api: thin_tess,
    })
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
