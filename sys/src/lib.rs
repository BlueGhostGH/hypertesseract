#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// I have no idea why this is needed..
#[cfg(windows)]
#[allow(unused_imports)]
use openssl_sys;

pub mod leptonica;

// line 76 - baseapi.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessBaseAPI
{
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ETEXT_DESC
{
    _unused: [u8; 0],
}

// pub const TessPageSegMode_PSM_OSD_ONLY: TessPageSegMode = 0;
// pub const TessPageSegMode_PSM_AUTO_OSD: TessPageSegMode = 1;
// pub const TessPageSegMode_PSM_AUTO_ONLY: TessPageSegMode = 2;
// pub const TessPageSegMode_PSM_AUTO: TessPageSegMode = 3;
// pub const TessPageSegMode_PSM_SINGLE_COLUMN: TessPageSegMode = 4;
// pub const TessPageSegMode_PSM_SINGLE_BLOCK_VERT_TEXT: TessPageSegMode = 5;
pub const TessPageSegMode_PSM_SINGLE_BLOCK: TessPageSegMode = 6;
pub const TessPageSegMode_PSM_SINGLE_LINE: TessPageSegMode = 7;
pub const TessPageSegMode_PSM_SINGLE_WORD: TessPageSegMode = 8;
// pub const TessPageSegMode_PSM_CIRCLE_WORD: TessPageSegMode = 9;
// pub const TessPageSegMode_PSM_SINGLE_CHAR: TessPageSegMode = 10;
pub const TessPageSegMode_PSM_SPARSE_TEXT: TessPageSegMode = 11;
// pub const TessPageSegMode_PSM_SPARSE_TEXT_OSD: TessPageSegMode = 12;
pub const TessPageSegMode_PSM_RAW_LINE: TessPageSegMode = 13;
pub const TessPageSegMode_PSM_COUNT: TessPageSegMode = 14;
pub type TessPageSegMode = ::std::os::raw::c_int;

extern "C" {
    // line 187 - capi.h
    pub fn TessBaseAPICreate() -> *mut TessBaseAPI;

    // line 224 - capi.h
    pub fn TessBaseAPIInit3(
        handle: *mut TessBaseAPI,
        datapath: *const ::std::os::raw::c_char,
        language: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    // line 201 - capi.h
    pub fn TessBaseAPISetVariable(
        handle: *mut TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    // line 253 - capi.h
    pub fn TessBaseAPISetPageSegMode(
        handle: *mut TessBaseAPI,
        mode: TessPageSegMode,
    );

    // line 268 - capi.h
    pub fn TessBaseAPISetImage2(
        handle: *mut TessBaseAPI,
        pix: *mut leptonica::Pix,
    );

    // line 307 - capi.h
    pub fn TessBaseAPIRecognize(
        handle: *mut TessBaseAPI,
        monitor: *mut ETEXT_DESC,
    ) -> ::std::os::raw::c_int;

    // line 188 - capi.h
    pub fn TessBaseAPIDelete(handle: *mut TessBaseAPI);
}
