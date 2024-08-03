use std::{borrow, ffi};

use super::Builder;

impl<Whitelist> Builder<((), Whitelist)>
{
    pub fn assume_numeric_input(self) -> Builder<(bool, Whitelist)>
    {
        let Builder {
            fields: ((), whitelist),
        } = self;

        Builder {
            fields: (true, whitelist),
        }
    }
}

impl<AssumeNumericInput> Builder<(AssumeNumericInput, ())>
{
    pub fn whitelist_str(
        self,
        whitelist: &str,
    ) -> Result<
        Builder<(AssumeNumericInput, borrow::Cow<'_, ffi::CStr>)>,
        ffi::NulError,
    >
    {
        let Builder {
            fields: (assume_numeric_input, ()),
        } = self;

        let whitelist = ffi::CString::new(whitelist)?;

        Ok(Builder {
            fields: (assume_numeric_input, borrow::Cow::from(whitelist)),
        })
    }

    pub fn whitelist_c_str(
        self,
        whitelist: &ffi::CStr,
    ) -> Builder<(AssumeNumericInput, borrow::Cow<'_, ffi::CStr>)>
    {
        let Builder {
            fields: (assume_numeric_input, ()),
        } = self;

        Builder {
            fields: (assume_numeric_input, borrow::Cow::from(whitelist)),
        }
    }
}

impl Builder<(bool, borrow::Cow<'_, ffi::CStr>)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist),
        } = self;

        super::build(assume_numeric_input, Some(whitelist))
    }
}

impl Builder<((), borrow::Cow<'_, ffi::CStr>)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist),
        } = self;

        super::build(false, Some(whitelist))
    }
}

impl Builder<(bool, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _),
        } = self;

        super::build(assume_numeric_input, None)
    }
}

impl Builder<((), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder { fields: (_, _) } = self;

        super::build(false, None)
    }
}
