use inquire::Confirm;
use std::fs;
use tracing::{info, instrument};

use crate::{
    format_package,
    utils::{
        paths::{c_packages, d_packages},
        regex_namespace, regex_package, regex_packagename,
        state::{Error, ErrorKind, Result},
    },
};

use super::UnlinkArgs;

#[instrument(skip(cmd))]
pub fn run(cmd: &UnlinkArgs) -> Result<bool> {
    let packages = &cmd.package;

    // RegEx
    let re_all = regex_package();
    let re_name = regex_packagename();
    let re_namespace = regex_namespace();
    let path: String;
    if let Some(cap) = re_all.captures(packages.as_str()) {
        let (_, [namespace, package, major, minor, patch]) = cap.extract();
        path = format_package!(namespace, package, major, minor, patch);
    } else if let Some(cap) = re_name.captures(packages.as_str()) {
        let (_, [namespace, package]) = cap.extract();
        path = format_package!(namespace, package);
    } else if let Some(cap) = re_namespace.captures(packages.as_str()) {
        let (_, [namespace]) = cap.extract();
        path = format_package!(namespace);
    } else {
        return Err(Error::empty(ErrorKind::PackageNotValid));
    }

    if !cmd.yes {
        match Confirm::new("Are you sure to delete this? This is irreversible.")
            .with_help_message(format!("You want to delete {packages}").as_str())
            .prompt()
        {
            Ok(_) => {
                fs::remove_dir_all(path)?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    } else {
        fs::remove_dir_all(path)?;
        Ok(true)
    }
}
