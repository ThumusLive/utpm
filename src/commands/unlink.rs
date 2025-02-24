use inquire::Confirm;
use owo_colors::OwoColorize;
use std::fs;
use tracing::instrument;

use crate::utils::{
    paths::d_packages,
    state::{Error, ErrorKind, Result},
};

use super::UnlinkArgs;

#[instrument]
pub fn run(cmd: &UnlinkArgs) -> Result<bool> {
    let mut new_namespace = String::from("local");
    if let Some(nspace) = &cmd.namespace {
        new_namespace = nspace.to_owned();
    }
    if let Some(ver) = &cmd.version {
        if cmd.name.is_none() {
            return Err(Error::empty(ErrorKind::Namespace));
        }
        let ans = if !(cmd.yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!(
                        "You want to erase {}/{}",
                        cmd.name.clone().unwrap(),
                        ver.to_string()
                    )
                    .as_str(),
                )
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            return Ok(false);
        }

        fs::remove_dir_all(
            d_packages()?
                + format!(
                    "/{}/{}/{}",
                    new_namespace,
                    cmd.name.clone().unwrap(),
                    ver.to_string()
                )
                .as_str(),
        )?;
    } else if cmd.delete_namespace {
        let ans = if !(cmd.yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(
                    format!("You want to erase @{new_namespace}, the namespace").as_str(),
                )
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            return Ok(false);
        }

        fs::remove_dir_all(d_packages()? + format!("/{new_namespace}").as_str())?;
    } else if let Some(nm) = &cmd.name {
        let ans = if !(cmd.yes) {
            Confirm::new("Are you sure to delete this? This is irreversible.")
                .with_help_message(format!("You want to erase {}", nm).as_str())
                .prompt()
        } else {
            Ok(true)
        };

        let bool = ans?;
        if !bool {
            return Ok(false);
        }

        fs::remove_dir_all(d_packages()? + format!("/{}/{}", new_namespace, nm).as_str())?;
    }
    println!("{}", "Removed!".bold());

    Ok(true)
}
