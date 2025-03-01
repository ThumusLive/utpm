use shadow_rs::shadow;
shadow!(build);

pub mod commands;
pub mod utils;

use std::{env, str::FromStr};

use clap::Parser;
#[cfg(feature = "add")]
use commands::add;
#[cfg(feature = "bulk_delete")]
use commands::bulk_delete;
#[cfg(feature = "clone")]
use commands::clone;
#[cfg(feature = "delete")]
use commands::delete;
#[cfg(feature = "generate")]
use commands::generate;
#[cfg(feature = "init")]
use commands::init;
#[cfg(feature = "install")]
use commands::install;
#[cfg(feature = "link")]
use commands::link;
#[cfg(feature = "list")]
use commands::list;
#[cfg(feature = "path")]
use commands::package_path;
#[cfg(feature = "publish")]
use commands::publish;
#[cfg(feature = "tree")]
use commands::tree;
#[cfg(feature = "unlink")]
use commands::unlink;
#[cfg(any(
    feature = "tree",
    feature = "list",
    feature = "path",
    feature = "unlink",
    feature = "bulk_delete"
))]
use commands::Packages;
//todo: workspace
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
use commands::Workspace;
use commands::{Cli, Commands};

use utils::state::Error;

use tracing::{error, instrument, level_filters::LevelFilter};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[instrument]
fn main() {
    let x = Cli::parse();

    // Fetching variables from the environment.
    let debug_str: String = match env::var("UTPM_DEBUG") {
        Err(_) => "warn".into(),
        Ok(val) => val,
    };

    // Transform the env var into a levelfilter to
    // filter logs from the tracing
    let level_filter: LevelFilter = match LevelFilter::from_str(debug_str.as_str()) {
        Ok(val) => val,
        Err(_) => LevelFilter::WARN,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(if let Some(debug) = x.verbose {
                debug
            } else {
                level_filter
            }),
        )
        .init();

    let res: Result<bool, Error> = match &x.command {
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
        Commands::Workspace(w) => match w {
            #[cfg(feature = "link")]
            Workspace::Link(cmd) => link::run(cmd, None, true),
            #[cfg(feature = "install")]
            Workspace::Install(cmd) => install::run(cmd),
            #[cfg(feature = "add")]
            Workspace::Add(cmd) => add::run(&mut cmd.clone()),
            #[cfg(feature = "delete")]
            Workspace::Delete(cmd) => delete::run(&mut cmd.clone()),
            #[cfg(feature = "init")]
            Workspace::Init(cmd) => init::run(&mut cmd.clone()),
            #[cfg(feature = "publish")]
            Workspace::Publish(cmd) => publish::run(cmd),
            #[cfg(feature = "clone")]
            Workspace::Clone(cmd) => clone::run(cmd),
        },
        #[cfg(any(
            feature = "tree",
            feature = "list",
            feature = "path",
            feature = "unlink",
            feature = "bulk_delete"
        ))]
        Commands::Packages(p) => match p {
            // Maybe a move command to change namespace? Or name or version
            #[cfg(feature = "tree")]
            Packages::Tree(cmd) => tree::run(cmd),
            #[cfg(feature = "list")]
            Packages::List(cmd) => list::run(cmd),
            #[cfg(feature = "path")]
            Packages::Path => package_path::run(),
            #[cfg(feature = "unlink")]
            Packages::Unlink(cmd) => unlink::run(cmd),
            #[cfg(feature = "bulk_delete")]
            Packages::BulkDelete(cmd) => bulk_delete::run(cmd),
        },
        #[cfg(feature = "generate")]
        Commands::Generate(cmd) => generate::run(cmd),
    };

    match res {
        Ok(_) => {}
        Err(val) => error!("{}", val),
    }
}
