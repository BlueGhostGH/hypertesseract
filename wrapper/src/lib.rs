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
    base_api: thin::Tesseract,
}

impl Tesseract
{
    pub fn builder() -> builder::Builder<((), (), (), ())>
    {
        builder::Builder::default()
    }

    pub fn recognize_text_cloned<'buf, I>(mut self, image: I) -> Result<String>
    where
        I: Into<Image<'buf>>,
    {
        // `rust-analyzer` refuses to give inlay hints..
        let image: Image<'buf> = image.into();

        let mut pix = thin::leptonica::Pix::create(
            image.width().try_into()?,
            image.height().try_into()?,
            image.depth().into(),
        );
        pix.set_data(image.buffer())?;
        self.base_api.set_image(&mut pix);

        self.base_api.recognize()?;

        let text = self.base_api.get_utf8_text()?;
        let text = text.as_ref().to_str()?.to_owned();

        Ok(text)
    }
}
