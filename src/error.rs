pub type ErrResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl<E> From<E> for Error
where
    E: std::fmt::Display,
{
    fn from(value: E) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

pub trait Context<T> {
    fn context(self, info: &str) -> Self;
    fn log(self) -> Self;
}

impl<T> Context<T> for ErrResult<T> {
    fn context(mut self, info: &str) -> Self {
        if let Err(error) = &mut self {
            error.message += " => ";
            error.message += info;
        }
        self
    }

    fn log(self) -> Self {
        if let Err(error) = &self {
            log::error!("{}", &error.message);
        }
        self
    }
}
