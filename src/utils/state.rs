use owo_colors::OwoColorize;
use semver::Version;
use std::{fmt, io::Error as IError};

/// All errors implemented in utpm
#[derive(Debug)]
pub enum ErrorKind {
    UnknowError(String),

    CurrentDir,
    CreationDir,
    HomeDir,

    Namespace,
    ConfigFile,
    AlreadyExist(String, Version, String),

    IO,
    Questions,
    Git,
    SemVer,
}


impl ErrorKind {
    pub fn message(&self) -> String {
        match self {
            ErrorKind::CurrentDir => "There is no current directory set.".into(),
            ErrorKind::CreationDir => "We can't create the directory.".into(),
            ErrorKind::HomeDir => "There is no home directory set.".into(),
            ErrorKind::Namespace => {
                "You need to provide at least a namespace or the name of the package".into()
            }
            ErrorKind::ConfigFile => {
                "There is no typst.toml in this directory. Try to `utpm create -p` to create a package.".into()
            }
            ErrorKind::AlreadyExist(name, version, info) => format!("This package ({name}:{version}) already exist!\n{info} Put --force to force the copy or change the version in 'typst.toml'"),
            ErrorKind::UnknowError(s) => s.into(),
            _ => "".into(),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Error {
    kind: ErrorKind,
    message: Option<String>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message: Some(message),
        }
    }
    pub fn empty(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }
    pub fn to_str(&self) -> String {
        let kind_message = format!("{} Error", self.kind.to_string());
        if let Some(message) = &self.message {
            format!("{}: {}", kind_message.bold().red(), message)
        } else {
            format!("{}: {}", kind_message.bold().red(), self.kind.message())
        }
    }

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}


//TODO: impl errors.

impl From<IError> for Error {
    fn from(err: IError) -> Error {
        Error::new(ErrorKind::IO, err.to_string())
    }
}

impl From<inquire::InquireError> for Error {
    fn from(err: inquire::InquireError) -> Error {
        Error::new(ErrorKind::Questions, err.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::new(ErrorKind::Git, err.to_string())
    }
}

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Error {
        Error::new(ErrorKind::SemVer, err.to_string())
    }
}
