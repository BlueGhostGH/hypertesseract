use std::ffi;

const fn as_variable_name(bytes: &'static [u8]) -> &'static ffi::CStr
{
    unsafe { ffi::CStr::from_bytes_with_nul_unchecked(bytes) }
}

pub(crate) const ASSUME_NUMERIC_INPUT: &'static ffi::CStr =
    as_variable_name(b"classify_bln_numeric_mode\0");

pub(crate) const WHITELIST: &'static ffi::CStr =
    as_variable_name(b"tessedit_char_whitelist\0");
