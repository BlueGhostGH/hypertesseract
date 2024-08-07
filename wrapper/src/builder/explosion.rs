use std::{borrow, ffi};

use super::{language, page_seg_mode, Builder};

impl<Whitelist, Language, PageSegMode>
    Builder<((), Whitelist, Language, PageSegMode)>
{
    pub fn assume_numeric_input(
        self,
    ) -> Builder<(bool, Whitelist, Language, PageSegMode)>
    {
        let Builder {
            fields: ((), whitelist, language, page_seg_mode),
        } = self;

        Builder {
            fields: (true, whitelist, language, page_seg_mode),
        }
    }
}

impl<AssumeNumericInput, Language, PageSegMode>
    Builder<(AssumeNumericInput, (), Language, PageSegMode)>
{
    pub fn whitelist_str(
        self,
        whitelist: &str,
    ) -> Result<
        Builder<(
            AssumeNumericInput,
            borrow::Cow<'_, ffi::CStr>,
            Language,
            PageSegMode,
        )>,
        ffi::NulError,
    >
    {
        let Builder {
            fields: (assume_numeric_input, (), language, page_seg_mode),
        } = self;

        let whitelist = ffi::CString::new(whitelist)?;

        Ok(Builder {
            fields: (
                assume_numeric_input,
                borrow::Cow::from(whitelist),
                language,
                page_seg_mode,
            ),
        })
    }

    pub fn whitelist_c_str(
        self,
        whitelist: &ffi::CStr,
    ) -> Builder<(
        AssumeNumericInput,
        borrow::Cow<'_, ffi::CStr>,
        Language,
        PageSegMode,
    )>
    {
        let Builder {
            fields: (assume_numeric_input, (), language, page_seg_mode),
        } = self;

        Builder {
            fields: (
                assume_numeric_input,
                borrow::Cow::from(whitelist),
                language,
                page_seg_mode,
            ),
        }
    }
}

impl<AssumeNumericInput, Whitelist, PageSegMode>
    Builder<(AssumeNumericInput, Whitelist, (), PageSegMode)>
{
    pub fn language(
        self,
        language: language::Language,
    ) -> Builder<(
        AssumeNumericInput,
        Whitelist,
        language::Language,
        PageSegMode,
    )>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, (), page_seg_mode),
        } = self;

        Builder {
            fields: (assume_numeric_input, whitelist, language, page_seg_mode),
        }
    }
}

impl<AssumeNumericInput, Whitelist, Language>
    Builder<(AssumeNumericInput, Whitelist, Language, ())>
{
    pub fn page_seg_mode(
        self,
        page_seg_mode: page_seg_mode::PageSegMode,
    ) -> Builder<(
        AssumeNumericInput,
        Whitelist,
        Language,
        page_seg_mode::PageSegMode,
    )>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, language, ()),
        } = self;

        Builder {
            fields: (assume_numeric_input, whitelist, language, page_seg_mode),
        }
    }
}

// 0 0 0 0
impl Builder<((), (), (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, _, _, _),
        } = self;

        super::build(false, None, None, None)
    }
}

// 1 0 0 0
impl Builder<(bool, (), (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, _, _),
        } = self;

        super::build(assume_numeric_input, None, None, None)
    }
}

// 0 1 0 0
impl Builder<((), borrow::Cow<'_, ffi::CStr>, (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, _, _),
        } = self;

        super::build(false, Some(whitelist), None, None)
    }
}

// 1 1 0 0
impl Builder<(bool, borrow::Cow<'_, ffi::CStr>, (), ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, _, _),
        } = self;

        super::build(assume_numeric_input, Some(whitelist), None, None)
    }
}

// 0 0 1 0
impl Builder<((), (), language::Language, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, _, language, _),
        } = self;

        super::build(false, None, Some(language), None)
    }
}

// 1 0 1 0
impl Builder<(bool, (), language::Language, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, language, _),
        } = self;

        super::build(assume_numeric_input, None, Some(language), None)
    }
}

// 0 1 1 0
impl Builder<((), borrow::Cow<'_, ffi::CStr>, language::Language, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, language, _),
        } = self;

        super::build(false, Some(whitelist), Some(language), None)
    }
}

// 1 1 1 0
impl Builder<(bool, borrow::Cow<'_, ffi::CStr>, language::Language, ())>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, language, _),
        } = self;

        super::build(
            assume_numeric_input,
            Some(whitelist),
            Some(language),
            None,
        )
    }
}

// 0 0 0 1
impl Builder<((), (), (), page_seg_mode::PageSegMode)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, _, _, page_seg_mode),
        } = self;

        super::build(false, None, None, Some(page_seg_mode))
    }
}

// 1 0 0 1
impl Builder<(bool, (), (), page_seg_mode::PageSegMode)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, _, page_seg_mode),
        } = self;

        super::build(assume_numeric_input, None, None, Some(page_seg_mode))
    }
}

// 0 1 0 1
impl
    Builder<(
        (),
        borrow::Cow<'_, ffi::CStr>,
        (),
        page_seg_mode::PageSegMode,
    )>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, _, page_seg_mode),
        } = self;

        super::build(false, Some(whitelist), None, Some(page_seg_mode))
    }
}

// 1 1 0 1
impl
    Builder<(
        bool,
        borrow::Cow<'_, ffi::CStr>,
        (),
        page_seg_mode::PageSegMode,
    )>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, _, page_seg_mode),
        } = self;

        super::build(
            assume_numeric_input,
            Some(whitelist),
            None,
            Some(page_seg_mode),
        )
    }
}

// 0 0 1 1
impl Builder<((), (), language::Language, page_seg_mode::PageSegMode)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, _, language, page_seg_mode),
        } = self;

        super::build(false, None, Some(language), Some(page_seg_mode))
    }
}

// 1 0 1 1
impl Builder<(bool, (), language::Language, page_seg_mode::PageSegMode)>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, _, language, page_seg_mode),
        } = self;

        super::build(
            assume_numeric_input,
            None,
            Some(language),
            Some(page_seg_mode),
        )
    }
}

// 0 1 1 1
impl
    Builder<(
        (),
        borrow::Cow<'_, ffi::CStr>,
        language::Language,
        page_seg_mode::PageSegMode,
    )>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (_, whitelist, language, page_seg_mode),
        } = self;

        super::build(
            false,
            Some(whitelist),
            Some(language),
            Some(page_seg_mode),
        )
    }
}

// 1 1 1 1
impl
    Builder<(
        bool,
        borrow::Cow<'_, ffi::CStr>,
        language::Language,
        page_seg_mode::PageSegMode,
    )>
{
    pub fn build(self) -> Result<crate::Tesseract, crate::Error>
    {
        let Builder {
            fields: (assume_numeric_input, whitelist, language, page_seg_mode),
        } = self;

        super::build(
            assume_numeric_input,
            Some(whitelist),
            Some(language),
            Some(page_seg_mode),
        )
    }
}
