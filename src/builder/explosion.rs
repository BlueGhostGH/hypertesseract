use std::{borrow, ffi};

use super::{language, Builder};

impl<Whitelist, Language> Builder<((), Whitelist, Language)>
{
    pub fn assume_numeric_input(self) -> Builder<(bool, Whitelist, Language)>
    {
        let Builder {
            fields: ((), whitelist, language),
        } = self;

        Builder {
            fields: (true, whitelist, language),
        }
    }
}

impl<AssumeNumericInput, Language> Builder<(AssumeNumericInput, (), Language)>
{
    pub fn whitelist_str(
        self,
        whitelist: &str,
    ) -> Result<
        Builder<(AssumeNumericInput, borrow::Cow<'_, ffi::CStr>, Language)>,
        ffi::NulError,
    >
    {
        let Builder {
            fields: (assume_numeric_input, (), language),
        } = self;

        let whitelist = ffi::CString::new(whitelist)?;

        Ok(Builder {
            fields: (
                assume_numeric_input,
                borrow::Cow::from(whitelist),
                language,
            ),
        })
    }

    pub fn whitelist_c_str(
        self,
        whitelist: &ffi::CStr,
    ) -> Builder<(AssumeNumericInput, borrow::Cow<'_, ffi::CStr>, Language)>
    {
        let Builder {
            fields: (assume_numeric_input, (), language),
        } = self;

        Builder {
            fields: (
                assume_numeric_input,
                borrow::Cow::from(whitelist),
                language,
            ),
        }
    }
}

impl<AssumeNumericInput, Whitelist> Builder<(AssumeNumericInput, Whitelist, ())>
{
    pub fn language(
        self,
        language: language::Language,
    ) -> Builder<(AssumeNumericInput, Whitelist, language::Language)>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, ()),
        } = self;

        Builder {
            fields: (assume_numeric_input, whitelist, language),
        }
    }
}

impl Builder<(bool, borrow::Cow<'_, ffi::CStr>, language::Language)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, language),
        } = self;

        super::build(assume_numeric_input, Some(whitelist), Some(language))
    }
}

impl Builder<(bool, borrow::Cow<'_, ffi::CStr>, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, _),
        } = self;

        super::build(assume_numeric_input, Some(whitelist), None)
    }
}

impl Builder<((), borrow::Cow<'_, ffi::CStr>, language::Language)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, language),
        } = self;

        super::build(false, Some(whitelist), Some(language))
    }
}

impl Builder<((), borrow::Cow<'_, ffi::CStr>, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, _),
        } = self;

        super::build(false, Some(whitelist), None)
    }
}

impl Builder<(bool, (), language::Language)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, language),
        } = self;

        super::build(assume_numeric_input, None, Some(language))
    }
}

impl Builder<(bool, (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, _),
        } = self;

        super::build(assume_numeric_input, None, None)
    }
}

impl Builder<((), (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder { fields: (_, _, _) } = self;

        super::build(false, None, None)
    }
}
