mod builder;

mod error;
mod image;

pub use builder::language::Language;
pub use builder::page_seg_mode::PageSegMode;

pub use error::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

pub use image::Image;

#[derive(Debug)]
pub struct Tesseract
{
    _base_api: thin::Tesseract,
}

impl Tesseract
{
    pub fn builder() -> builder::Builder<((), (), (), ())>
    {
        builder::Builder::default()
    }

    pub fn load_image<'buf, I>(&mut self, image: I) -> Result<()>
    where
        I: Into<image::Image<'buf>>,
    {
        let image = image.into();

        let mut pix = thin::leptonica::Pix::create(
            image.width().try_into()?,
            image.height().try_into()?,
            image.depth().into(),
        );

        pix.set_data(image.buffer())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn builder_compiles()
    {
        let mut t = crate::Tesseract::builder()
            .assume_numeric_input()
            .whitelist_str("abcdef")
            .unwrap()
            .language(crate::Language::English)
            .page_seg_mode(crate::PageSegMode::SingleLine)
            .build()
            .unwrap();

        const WIDTH: usize = 128;
        const HEIGHT: usize = 128;
        let buffer = vec![255; WIDTH * HEIGHT * 4];

        t.load_image(crate::Image::RGBA8 {
            buffer: &buffer,
            width: WIDTH as u32,
            height: HEIGHT as u32,
        })
        .unwrap();

        assert!(true);
    }
}
