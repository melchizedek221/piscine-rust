// use std::fmt;
// use std::fmt::Display;
// use std::error::Error;

pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn Error>),
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Fail to parse todo"),
            ParseErr::Malformed(_) => write!(f, "Fail to parse todo"),
        }
    }
}

pub struct ReadErr {
    // expected public fields
    child_err: Box<dyn Error>,

}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(err) => Some(err.as_ref()),
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}
