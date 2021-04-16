use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    MissingRequiredVaribles { missing_variables: Vec<String> },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingRequiredVaribles { missing_variables } => {
                write!(f, "missing required variables: [\n")?;
                for missing_variable in missing_variables {
                    write!(f, "{}", missing_variable)?;
                }
                write!(f, "]")
            }
        }
    }
}
