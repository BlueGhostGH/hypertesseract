fn main() -> Result<(), Box<dyn ::std::error::Error>>
{
    const WIDTH: usize = 128;
    const HEIGHT: usize = 128;
    let buffer = vec![255; WIDTH * HEIGHT * 4];

    let mut t = tesseract::Tesseract::builder()
        .assume_numeric_input()
        .whitelist_str("abcdef")?
        .language(tesseract::Language::English)
        .page_seg_mode(tesseract::PageSegMode::SingleLine)
        .build()?;

    t.load_image(tesseract::Image::RGBA8 {
        buffer: &buffer,
        width: WIDTH as u32,
        height: HEIGHT as u32,
    })?;

    Ok(())
}
