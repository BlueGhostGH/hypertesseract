#[derive(Debug)]
pub enum Image<'buf>
{
    RGBA8
    {
        buffer: &'buf [u8],
        width: u32,
        height: u32,
    },
}

impl Image<'_>
{
    pub(crate) fn width(&self) -> u32
    {
        match self {
            Image::RGBA8 { width, .. } => *width,
        }
    }

    pub(crate) fn height(&self) -> u32
    {
        match self {
            Image::RGBA8 { height, .. } => *height,
        }
    }

    pub(crate) fn depth(&self) -> u8
    {
        match self {
            Image::RGBA8 { .. } => 32,
        }
    }

    pub(crate) fn buffer(&self) -> &[u8]
    {
        match self {
            Image::RGBA8 { buffer, .. } => buffer,
        }
    }
}

#[cfg(feature = "image")]
impl<'buf> From<&'buf image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>
    for Image<'buf>
{
    fn from(image: &'buf image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self
    {
        Image::RGBA8 {
            buffer: image.as_raw(),
            width: image.width(),
            height: image.height(),
        }
    }
}
