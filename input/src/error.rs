use std::fmt;

#[derive(Debug)]
pub enum AlgebraError {
    EmptyEquation,
    EmptyEquationHeader,
    UnknownToken,

    #[doc(hidden)]
    __NoneExhausive,
}

impl fmt::Display for AlgebraError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
       match self {
            AlgebraError::EmptyEquation => write!(formatter, "the equation that is given is empty"),
            AlgebraError::EmptyEquationHeader => write!(formatter, "the equation that is given has no header"),
            AlgebraError::UnknownToken => write!(formatter, "could not find a good token"),
            AlgebraError::__NoneExhausive => write!(formatter, "an unknown error occured"),
       }
    }
}
