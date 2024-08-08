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

    pub fn load_image<'buf, I>(&mut self, image: I) -> Result<&mut Self>
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

        self.base_api.set_image(&mut pix);

        // TODO: Store the pix somewhere
        ::std::mem::forget(pix);

        Ok(self)
    }

    pub fn recognize(&mut self) -> Result<&mut Self>
    {
        self.base_api.recognize()?;

        Ok(self)
    }

    // TODO: Figure out correct lifetime
    pub fn get_text(&mut self) -> Result<String>
    {
        Ok(self.base_api.get_utf8_text()?.as_ref().to_str()?.to_owned())
    }
}
