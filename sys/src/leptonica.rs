#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pix
{
    pub _address: u8,
}

pub type PIX = Pix;

extern "C" {
    // line 1458 - allheaders.h
    pub fn pixCreate(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
    ) -> *mut PIX;

    // line 1508 - allheaders.h
    pub fn pixSetData(
        pix: *mut PIX,
        data: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;

    // line 1465 - allheaders.h
    pub fn pixDestroy(ppix: *mut *mut PIX);
}
