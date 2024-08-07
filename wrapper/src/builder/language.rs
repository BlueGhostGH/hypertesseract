use std::ffi;

#[derive(Debug)]
pub enum Language
{
    English,
}

impl Language
{
    pub(super) fn as_lang_code_c_str(self) -> &'static ffi::CStr
    {
        let code: &[u8] = match self {
            Language::English => b"eng\0",
        };

        unsafe { ffi::CStr::from_bytes_with_nul_unchecked(code) }
    }
}
