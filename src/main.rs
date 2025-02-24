use shadow_rs::shadow;
shadow!(build);

pub mod commands;
pub mod utils;

use std::{env, str::FromStr};

use clap::Parser;
use commands::{
    add, bulk_delete, clone, create, delete, generate, install, link, list, package_path, tree,
    unlink, Cli, Commands, Packages, Workspace,
};

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
        Commands::Workspace(w) => match w {
            Workspace::Link(cmd) => link::run(cmd, None, true),
            Workspace::Create(cmd) => create::run(&mut cmd.clone()),
            Workspace::Install(cmd) => install::run(cmd),
            Workspace::Add(cmd) => add::run(&mut cmd.clone()),
            Workspace::Delete(cmd) => delete::run(&mut cmd.clone()),
            Workspace::Init(cmd) => create::run(&mut cmd.clone()),
            Workspace::Publish => todo!(),
            Workspace::Clone(cmd) => clone::run(cmd),
        },
        Commands::Packages(p) => match p {
            Packages::Tree(cmd) => tree::run(cmd),
            Packages::List(cmd) => list::run(cmd),
            Packages::Path => package_path::run(),
            Packages::Unlink(cmd) => unlink::run(cmd),
            Packages::BulkDelete(cmd) => bulk_delete::run(cmd),
        },
        Commands::Generate(cmd) => generate::run(cmd),
    };

    match res {
        Ok(_) => {}
        Err(val) => error!("{}", val),
    }
}
