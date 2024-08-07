#[derive(Debug)]
pub enum PageSegMode
{
    SingleBlock,
    SingleLine,
    SingleWord,
    SparseText,
    RawLine,
}

impl PageSegMode
{
    pub(super) fn as_tess_page_seg_mode(self) -> sys::TessPageSegMode
    {
        use sys::{
            TessPageSegMode_PSM_RAW_LINE, TessPageSegMode_PSM_SINGLE_BLOCK,
            TessPageSegMode_PSM_SINGLE_LINE, TessPageSegMode_PSM_SINGLE_WORD,
            TessPageSegMode_PSM_SPARSE_TEXT,
        };

        match self {
            PageSegMode::SingleBlock => TessPageSegMode_PSM_SINGLE_BLOCK,
            PageSegMode::SingleLine => TessPageSegMode_PSM_SINGLE_LINE,
            PageSegMode::SingleWord => TessPageSegMode_PSM_SINGLE_WORD,
            PageSegMode::SparseText => TessPageSegMode_PSM_SPARSE_TEXT,
            PageSegMode::RawLine => TessPageSegMode_PSM_RAW_LINE,
        }
    }
}
