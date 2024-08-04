use std::{borrow, ffi};

use crate::thin;

// NOTE: Gotta hide the combinatorial explosion..
mod explosion;
pub(crate) mod language;

// Tuple goes as follows:
// [0] - assume_numeric_input
// [1] - whitelist
// [2] - language
#[derive(Debug)]
pub struct Builder<Fields = ((), (), ())>
{
    fields: Fields,
}

pub fn build(
    assume_numeric_input: bool,
    whitelist: Option<borrow::Cow<'_, ffi::CStr>>, // TODO: Maybe figure something better
    language: Option<language::Language>,
) -> Result<crate::Tesseract, crate::Error>
{
    let mut thin_tess = crate::thin::Tesseract::create();

    thin_tess.init(language.map(language::Language::as_lang_code_c_str))?;

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

impl ::std::default::Default for Builder<((), (), ())>
{
    fn default() -> Self
    {
        Builder {
            fields: ((), (), ()),
        }
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
