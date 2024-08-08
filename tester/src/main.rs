#![feature(string_remove_matches)]

use hypertesseract::{Language, PageSegMode, Tesseract};

const WHITELIST: &'static str = "0123456789'/";

fn main() -> Result<(), Box<dyn ::std::error::Error>>
{
    let image = image::ImageReader::open("../test.bmp")?
        .decode()?
        .into_rgba8();

    let text = Tesseract::builder()
        .assume_numeric_input()
        .whitelist_str(WHITELIST)?
        .language(Language::English)
        .page_seg_mode(PageSegMode::SingleLine)
        .build()?
        .load_image(&image)?
        .recognize()?
        .get_text()?;

    // We need to perform some additional operations
    // due to the exotic font
    let mut text = text.replace('/', "7");
    text.remove_matches('\'');

    let score: u32 = text.trim().parse()?;

    assert_eq!(score, 9983744);
    println!("{score}");

    Ok(())
}
