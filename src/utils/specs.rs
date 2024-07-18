use core::fmt;
use std::fs::{self, read_to_string};

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::EnumString;

use clap::{ValueEnum, self};

/// The file `typst.toml` itself
#[derive(Serialize, Deserialize)]
pub struct TypstConfig {
    /// Base of typst package system
    pub package: Package,
    /// An extra for utpm (v2 only)
    pub utpm: Option<Extra>,

    /// Base on the new structure of typst package
    pub tool: Option<Tools>,

    pub template: Option<Template>,
}

impl TypstConfig {
    /// Load the configuration from a file
    pub fn load(path: &String) -> Self {
        toml::from_str(
            read_to_string(path)
                .expect("Should have read the file")
                .as_str(),
        )
        .unwrap()
    }

    /// Write a file
    pub fn write(&mut self, path: &String) {
        let form = toml::to_string_pretty(&self).unwrap();
        fs::write(path, form).expect("aaa");
    }

    /// Create a typstConfig
    pub fn new(package: Package, extra: Option<Extra>, template: Option<Template>) -> Self {
        Self {
            package,
            utpm: None, // We allow only readable versions.
            template,
            tool: Some(Tools::new(extra)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub path: String,
    pub entrypoint: String,
    pub thumbnail: Option<String>,
}

impl Template {
    pub fn new(path: String, entrypoint: String, thumbnail: Option<String>) -> Self {
        Self {
            path,
            entrypoint,
            thumbnail,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
/// Represent a package from the official `typst.toml`
/// See https://github.com/typst/packages
pub struct Package {
    // Required
    /// The name of the package
    pub name: String,
    /// The version (using semver)
    pub version: Version,
    /// Where is your main file
    pub entrypoint: String,

    // Not required with local packages
    /// The authors of the package
    pub authors: Option<Vec<String>>,
    /// The license
    pub license: Option<String>,
    /// A little description for your users
    pub description: Option<String>,

    // Not required
    /// A link to your repository
    pub repository: Option<String>,
    /// The link to your website
    pub homepage: Option<String>,
    /// A list of keywords to research your package
    pub keywords: Option<Vec<String>>,

    pub categories: Option<Vec<Categorie>>,
    pub disciplines: Option<Vec<Discipline>>,

    /// A minimum version of the compiler (for typst)
    pub compiler: Option<Version>,
    /// A list of excludes files
    pub exclude: Option<Vec<String>>,
}

/// Default implementation of a package
impl Package {
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            version: Version::new(1, 0, 0),
            entrypoint: "main.typ".to_string(),

            authors: Some(vec![]),
            license: None,
            description: None,

            repository: None,
            homepage: None,
            keywords: None,
            categories: Some(vec![]),
            disciplines: Some(vec![]),
            compiler: None,
            exclude: None,
        }
    }
}

#[derive(Debug, ValueEnum, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
#[clap(rename_all="lower")]
pub enum Discipline {    
    Agriculture,
    Anthropology,
    Archaeology,
    Architecture,
    Biology,
    Business,
    Chemistry,
    Communication,
    #[strum(serialize = "computer-science")]
    #[clap(rename_all="kebab-case")]
    ComputerScience,
    Design,
    Drawing,
    Economics,
    Education,
    Engineering,
    Fashion,
    Film,
    Geography,
    Geology,
    History,
    Journalism,
    Law,
    Linguistics,
    Literature,
    Mathematics,
    Medicine,
    Music,
    Painting,
    Philosophy,
    Photography,
    Physics,
    Politics,
    Psychology,
    Sociology,
    Theater,
    Theology,
    Transportation,

}

impl fmt::Display for Discipline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = format!("{:?}", self);
        if let Some(r) = s.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        write!(f, "{:?}", s)
    }
}

#[derive(Debug, ValueEnum, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString)]
#[clap(rename_all="lower")]
#[strum(serialize_all = "lowercase")]
pub enum Categorie {
    Components,
    Visualization,
    Model,
    Layout,
    Text,
    Languages,
    Scripting,
    Integration,
    Utility,
    Fun,

    Book,
    Report,
    Paper,
    Thesis,
    Poster,
    Flyer,
    Presentation,
    CV,
    Office,
}


impl fmt::Display for Categorie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = format!("{:?}", self);
        if let Some(r) = s.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        write!(f, "{:?}", s)
    }
}

/// A modify version of the `typst.toml` adding options to utpm
#[derive(Serialize, Deserialize, Clone)]
pub struct Extra {
    /// Basic system of version (it will increased over time to keep track of what change or not)
    pub version: Option<String>,
    /// The name of where you store your packages (default: local)
    pub namespace: Option<String>,

    /// List of url's for your dependencies (will be resolved with install command)
    pub dependencies: Option<Vec<String>>,
}

impl Extra {
    pub fn default() -> Self {
        Self {
            version: Some("3".into()),
            namespace: Some("local".to_string()),
            dependencies: None,
        }
    }

    pub fn new(version: Option<String>, namespace: Option<String>, dependencies: Option<Vec<String>>) -> Self {
        Self {
            version,
            namespace,
            dependencies
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tools {
    /// An extra for utpm (v3 only)
    pub utpm: Option<Extra>,
}

impl Tools {
    pub fn new(utpm: Option<Extra>) -> Self {
        Self { utpm }
    }
}
