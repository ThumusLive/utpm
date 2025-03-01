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
    General,

    Author,
    Website,
    Email,
    GithubHandle,

    Ident,
    License,

    Serialize,
    Deserialize,

    Manifest,

    NotEnoughArgs,

    // Clone
    PackageNotValid,
    PackageNotExist,
    ContentFound,
}

impl ErrorKind {
    // TODO: Remake this system
    /// Create a message when there isn't one provided (depreciated)
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
            ErrorKind::Manifest => "There is no `typst.toml` here!".into(),
            ErrorKind::AlreadyExist(name, version, info) => format!("This package ({name}:{version}) already exist!\n{info} Put --force to force the copy or change the version in 'typst.toml'"),
            ErrorKind::UnknowError(s) => s.into(),
            ErrorKind::NotEnoughArgs => "There is not enough args:".into(),
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
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(message.into()),
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
            format!("{}: {}", kind_message, message)
        } else {
            format!("{}: {}", kind_message, self.kind.message())
        }
    }

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message.is_none() {
            write!(f, "{}", self.to_str())
        } else {
            write!(
                f,
                "{}: {}\n{}",
                format!("{} Error", self.kind.to_string()),
                self.kind.message(),
                self.message.clone().unwrap()
            )
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

// From `https://github.com/tingerrr/typst-project/blob/e19fb3d68b10fce7d2366f4e5969edac6e2f7d34/src/manifest.rs#L182`
macro_rules! impl_from {
    ($err:ty => $var:ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Self {
                Error::new(ErrorKind::$var, err.to_string())
            }
        }
    };
}
impl_from!(semver::Error => SemVer);
#[cfg(any(feature = "install", feature = "clone", feature = "publish"))]
impl_from!(git2::Error => Git);
#[cfg(any(feature = "init", feature = "unlink"))]
impl_from!(inquire::InquireError => Questions);
impl_from!(IError => IO);
impl_from!(typst_project::manifest::Error => General);
impl_from!(typst_project::manifest::author::ParseAuthorError => Author);
impl_from!(typst_project::manifest::author::ParseWebsiteError => Website);
impl_from!(typst_project::manifest::author::ParseEmailError => Email);
impl_from!(typst_project::manifest::author::ParseGitHubHandleError => GithubHandle);
impl_from!(typst_project::manifest::ident::ParseIdentError => Ident);
impl_from!(typst_project::manifest::license::ParseLicenseError => License);

impl_from!(toml::ser::Error => Serialize);
impl_from!(toml::de::Error => Deserialize);
#[cfg(any(feature = "publish"))]
impl_from!(ignore::Error => General);
#[cfg(any(feature = "publish"))]
impl_from!(octocrab::Error => General); // todo
