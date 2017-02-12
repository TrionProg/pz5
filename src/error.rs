use std;

#[derive(Debug)]
pub enum Error{
    IOError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::IOError(ref e) => write!(f, "Byte order error:{}", e),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
/*
impl From<collada::Error> for Error {
    fn from(error:collada::Error) -> Error {
        Error::ColladaError(error)
    }
}
*/
