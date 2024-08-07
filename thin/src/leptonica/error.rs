#[derive(Debug)]
pub enum Error
{
    SetData(set_data::Error),
}

impl ::std::fmt::Display for Error
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
    {
        match self {
            Error::SetData(set_data_err) => {
                write!(f, "error while calling set_data: {set_data_err}")
            }
        }
    }
}

impl ::std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)>
    {
        match self {
            Error::SetData(set_data_err) => Some(set_data_err),
        }
    }
}

impl From<set_data::Error> for Error
{
    fn from(set_data_err: set_data::Error) -> Self
    {
        Error::SetData(set_data_err)
    }
}

pub(super) mod set_data
{

    #[derive(Debug)]
    pub enum Error
    {
        // TODO: Make more in depth/detailed
        FailedToSetData,
    }

    impl ::std::fmt::Display for Error
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
        {
            match self {
                Self::FailedToSetData => {
                    write!(f, "failed to set data for pix")
                }
            }
        }
    }

    impl ::std::error::Error for Error
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
        {
            match self {
                Self::FailedToSetData => None,
            }
        }
    }
}
