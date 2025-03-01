// Linker
#[cfg(feature = "add")]
pub mod add;
#[cfg(feature = "bulk_delete")]
pub mod bulk_delete;
#[cfg(feature = "clone")]
pub mod clone;
#[cfg(feature = "delete")]
pub mod delete;
#[cfg(feature = "generate")]
pub mod generate;
#[cfg(feature = "init")]
pub mod init;
#[cfg(feature = "install")]
pub mod install;
#[cfg(feature = "link")]
pub mod link;
#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "path")]
pub mod package_path;
#[cfg(feature = "publish")]
pub mod publish;
#[cfg(feature = "tree")]
pub mod tree;
#[cfg(feature = "unlink")]
pub mod unlink;

#[cfg(any(feature = "clone", feature = "publish"))]
use std::path::PathBuf;

use clap::{Parser, Subcommand};
#[cfg(feature = "generate")]
use clap_complete::Shell;
use tracing::level_filters::LevelFilter;

#[cfg(feature = "init")]
use typst_project::manifest::{categories::Category, disciplines::Discipline};

use crate::build;

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "init")]
pub struct InitArgs {
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
    #[clap(value_delimiter = ',')]
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
    #[clap(value_delimiter = ',')]
    keywords: Option<Vec<String>>,

    /// Minimum compiler version
    #[arg(short, long)]
    compiler: Option<semver::Version>,

    /// Excludes files
    #[arg(short = 'x', long)]
    #[clap(value_delimiter = ',')]
    exclude: Option<Vec<String>>,

    /// Namespace to put your package
    #[arg(short = 'N', long)]
    namespace: Option<String>,

    /// Add examples file to your projects.
    #[arg(short = 'p', long)]
    populate: bool,

    /// Add categories to your typst.toml
    #[arg(short = 'C', long)]
    #[clap(value_delimiter = ',')]
    categories: Option<Vec<Category>>,

    /// Add disciplines to your typst.toml
    #[arg(short = 'D', long)]
    #[clap(value_delimiter = ',')]
    disciplines: Option<Vec<Discipline>>,

    /// Add a link to your template. Example: "./template.typ"
    #[arg(long, requires = "template")]
    template_path: Option<String>,

    #[arg(long, requires = "template")]
    template_entrypoint: Option<String>,

    #[arg(long)]
    template_thumbnail: Option<String>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "link")]
pub struct LinkArgs {
    /// Force the copy of the dir / creation of the symlink
    #[arg(short, long)]
    pub force: bool,

    /// Will create a symlink instead of copying
    #[arg(short, long)]
    pub no_copy: bool,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(any(feature = "list", feature = "tree"))]
pub struct ListTreeArgs {
    /// Will list all packages including @preview
    #[arg(short, long)]
    pub all: bool,

    /// List all subdirectory you want
    #[arg(short, long, num_args = 1..)]
    pub include: Option<Vec<String>>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "publish")]
pub struct PublishArgs {
    #[arg()]
    path: Option<PathBuf>,

    /// Add rules of .ignore files to the check
    #[arg(short = 'i', default_value_t = false)]
    ignore: bool,

    #[arg(short = 'g', default_value_t = true)]
    git_ignore: bool,

    #[arg(short = 't', default_value_t = true)]
    typst_ignore: bool,

    #[arg(short = 'G', default_value_t = true)]
    git_global_ignore: bool,

    #[arg(short = 'x', default_value_t = true)]
    git_exclude: bool,

    #[arg(short = 'c')]
    custom_ignore: Option<PathBuf>,

    /// Specify a message for the new commit.
    #[arg(short = 'm')]
    message: Option<String>,

    /// Won't create a PR on typst/packages
    #[arg(short = 'p', default_value_t = false)]
    prepare_only: bool,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "generate")]
pub struct GenerateArgs {
    /// The type of your shell
    #[arg(value_enum)]
    generator: Shell,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "clone")]
pub struct CloneArgs {
    /// The name of the package you want to clone
    #[arg()]
    pub package: String,

    /// Path to your dir
    #[arg()]
    pub path: Option<PathBuf>,

    /// Download the package without copying it.
    #[arg(short = 'd')]
    pub download_only: bool,

    /// Continue without veryfing anything.
    #[arg(short = 'f')]
    pub force: bool,

    /// Force the redownload of the package.
    #[arg(short = 'r')]
    pub redownload: bool,

    /// Create a symlink to the package clone (similar to link --no-copy)
    #[arg(short = 's')]
    pub symlink: bool,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "unlink")]
pub struct UnlinkArgs {
    /// The name of the package
    package: String,

    /// Confirm the deletion of a dir
    #[arg(short, long)]
    yes: bool,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "bulk_delete")]
pub struct BulkDeleteArgs {
    /// Names of your packages, use version with this syntax: mypackage:1.0.0
    #[clap(value_delimiter = ',')]
    names: Vec<String>,

    /// The namespace you want to bulk-delete
    #[arg(short, long)]
    namespace: Option<String>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "install")]
pub struct InstallArgs {
    /// If you want to install a specific package
    #[arg(num_args = 1..)]
    pub url: Option<String>,

    /// Passed force to all link commands
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "delete")]
pub struct DeleteArgs {
    /// URIs to remove.
    pub uri: Vec<String>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
#[cfg(feature = "add")]
pub struct AddArgs {
    /// The url or path of your repository.
    pub uri: Vec<String>,
}

/// Commands to use packages related to typst
#[derive(Subcommand, Debug, PartialEq)]
#[cfg(any(
    feature = "tree",
    feature = "list",
    feature = "path",
    feature = "unlink",
    feature = "bulk_delete"
))]
pub enum Packages {
    /// List all of packages from your dir, in a form of a tree
    #[command(visible_alias = "t")]
    #[cfg(feature = "tree")]
    Tree(ListTreeArgs),

    /// List all of packages from your dir, in a form of a list
    #[command(visible_alias = "l")]
    #[cfg(feature = "list")]
    List(ListTreeArgs),

    /// Display path to typst packages folder
    #[command(visible_alias = "p")]
    #[cfg(feature = "path")]
    Path,

    /// Delete package previously install with utpm
    #[command(visible_alias = "u")]
    #[cfg(feature = "unlink")]
    Unlink(UnlinkArgs),

    /// Delete multiple packages/namespace at once
    #[command(visible_alias = "bd")]
    #[cfg(feature = "bulk_delete")]
    BulkDelete(BulkDeleteArgs),
}

/// Commands to create, edit, delete your workspace for your package.
#[derive(Subcommand, Debug, PartialEq)]
#[cfg(any(
    feature = "link",
    feature = "init",
    feature = "install",
    feature = "add",
    feature = "delete",
    feature = "init",
    feature = "publish",
    feature = "clone"
))]
pub enum Workspace {
    /// Link your project to your dirs
    #[command(visible_alias = "l")]
    #[cfg(feature = "link")]
    Link(LinkArgs),

    /// Install all dependencies from the `typst.toml`
    #[command(visible_alias = "i")]
    #[cfg(feature = "install")]
    Install(InstallArgs),

    /// Add dependencies and then install them
    #[command(visible_alias = "a")]
    #[cfg(feature = "add")]
    Add(AddArgs),

    /// Delete dependencies
    #[command(visible_alias = "d")]
    #[cfg(feature = "delete")]
    Delete(DeleteArgs),

    /// Create your workspace to start a typst package
    #[cfg(feature = "init")]
    Init(InitArgs),

    /// Publish directly your packages to typst universe. (WIP)
    #[command(visible_alias = "p")]
    #[cfg(feature = "publish")]
    Publish(PublishArgs),

    /// Clone like a git clone packages from typst universe or your local directory
    #[command()]
    #[cfg(feature = "clone")]
    Clone(CloneArgs),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[command(subcommand)]
    #[command(visible_alias = "ws")]
    #[cfg(any(
        feature = "link",
        feature = "init",
        feature = "install",
        feature = "add",
        feature = "delete",
        feature = "init",
        feature = "publish",
        feature = "clone"
    ))]
    Workspace(Workspace),

    #[command(subcommand)]
    #[command(visible_alias = "pkg")]
    #[cfg(any(
        feature = "tree",
        feature = "list",
        feature = "path",
        feature = "unlink",
        feature = "bulk_delete"
    ))]
    Packages(Packages),

    /// Generate shell completion
    #[command(visible_alias = "gen")]
    #[cfg(feature = "generate")]
    Generate(GenerateArgs),
}

#[derive(Parser, Debug, PartialEq)]
#[cfg(feature = "nightly")]
#[command(author = "Thumus", version = build::COMMIT_HASH)]
/// An unofficial typst package manager for your projects.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Gives you more information, permit debug.
    #[arg(short = 'v', long)]
    pub verbose: Option<LevelFilter>,
}

#[derive(Parser, Debug, PartialEq)]
#[cfg(not(feature = "nightly"))]
#[command(author = "Thumus", version = build::PKG_VERSION)]
/// An unofficial typst package manager for your projects.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Gives you more information, permit debug.
    #[arg(short = 'v', long)]
    pub verbose: Option<LevelFilter>,
}
