// Linker
pub mod bulk_delete;
pub mod create;
pub mod install;
pub mod link;
pub mod tree;
pub mod package_path;
pub mod unlink;
pub mod list;

use clap::{Parser, Subcommand};

use crate::utils::specs::{Categorie, Discipline};

#[derive(Parser, Clone, Debug)]
pub struct CreateArgs {
    /// Desactivate interactive session
    #[arg(short = 'm', long, requires = "ni")]
    cli: bool,

    /// Force the creation of a file
    #[arg(short, long)]
    force: bool,

    /// Name of the project
    #[arg(short, long, group = "ni")]
    name: Option<String>,

    /// Version of the project
    #[arg(short, long, default_value_t=semver::Version::parse("1.0.0").unwrap())]
    version: semver::Version,

    /// Path to the main file of the project
    #[arg(short, long, default_value_t=String::from("main.typ"))]
    entrypoint: String,

    /// Authors of the project
    #[arg(short, long)]
    authors: Option<Vec<String>>,

    /// License
    #[arg(short, long)]
    license: Option<String>,

    /// A little description
    #[arg(short, long)]
    description: Option<String>,

    /// The link to your repository
    #[arg(short, long)]
    repository: Option<String>,

    /// Link to your homepage
    #[arg(short = 'H', long)]
    homepage: Option<String>,

    /// Keywords to find your project
    #[arg(short, long)]
    keywords: Option<Vec<String>>,

    /// Minimum compiler version
    #[arg(short, long)]
    compiler: Option<semver::Version>,

    /// Excludes files
    #[arg(short = 'x', long)]
    exclude: Option<Vec<String>>,

    /// Namespace
    #[arg(short = 'N', long)]
    namespace: Option<String>,

    /// Populate
    #[arg(short = 'p', long)]
    populate: bool,

    #[arg(short = 'C', long)]
    categories: Option<Vec<Categorie>>,

    #[arg(short = 'D', long)]
    disciplines: Option<Vec<Discipline>>,

    #[arg(long, requires = "template")]
    template_path: Option<String>,

    #[arg(long, requires = "template")]
    template_entrypoint: Option<String>,

    #[arg(long)]
    template_thumbnail: Option<String>,
}

#[derive(Parser, Clone, Debug)]
pub struct LinkArgs {
    /// Force the copy of the dir / creation of the symlink
    #[arg(short, long)]
    pub force: bool,

    /// Will create a symlink instead of copying
    #[arg(short, long)]
    pub no_copy: bool,
}

#[derive(Parser, Clone, Debug)]
pub struct UnlinkArgs {
    /// The name of the package
    name: Option<String>,

    /// Namespace, where your packages are install (default local)
    #[arg(short, long)]
    namespace: Option<String>,

    /// Do you want to delete the namespace or not
    #[arg(short, long)]
    delete_namespace: bool,

    /// The version you want to delete, if nothing has been provided it will be the whole package
    #[arg(short, long)]
    version: Option<semver::Version>,

    /// Confirm the deletion of a dir
    #[arg(short, long)]
    yes: bool,
}

#[derive(Parser, Clone, Debug)]
pub struct BulkDeleteArgs {
    /// Names of your packages, use version with this syntax: mypackage:1.0.0
    names: Vec<String>,

    /// The namespace you want to bulk-delete
    #[arg(short, long)]
    namespace: Option<String>,
}

#[derive(Parser, Clone, Debug)]
pub struct InstallArgs {
    /// If you want to install a specific package
    pub url: Option<String>,

    /// Passed force to all link commands
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a file for your project (typst.toml)
    Create(CreateArgs),
    /// Link your project to your dirs
    Link(LinkArgs),

    /// List all of packages from your dir, in a form of a tree
    Tree,

    /// List all of packages from your dir, in a form of a list
    List,

    /// Display path to typst packages folder
    PackagesPath,

    /// Delete package previously install with utpm
    Unlink(UnlinkArgs),

    /// Delete a bunch of packages
    BulkDelete(BulkDeleteArgs),

    /// Install all dependencies from the `typst.toml`
    Install(InstallArgs),
}

#[derive(Parser)]
#[command(author = "Thumus", version = "3.0.0")]
/// An unofficial typst package manager for your projects.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
