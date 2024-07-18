use owo_colors::OwoColorize;
use std::fs;

use crate::utils::{
    copy_dir_all,
    paths::{check_path_dir, d_packages, get_current_dir},
    state::{Error, ErrorKind, Result},
    symlink_all, specs::{Extra, TypstConfig},
};

use super::LinkArgs;

pub fn run(cmd: &LinkArgs, path: Option<String>) -> Result<bool> {
    let curr = path.unwrap_or(get_current_dir()?);

    let config = TypstConfig::load(&(curr.clone() + "/typst.toml"));
    let namespace = config
        .utpm
        .unwrap_or(Extra::default())
        .namespace
        .unwrap_or("local".into());

    let name = config.package.name;
    let version = config.package.version;
    let path = format!("{}/{}/{}/{}", d_packages(), namespace, name, version);
    let binding = "Info:".yellow();
    let info = binding.bold();
    if check_path_dir(&path) && !cmd.force {
        return Err(Error::empty(ErrorKind::AlreadyExist(
            name,
            version,
            format!("{}", info),
        )));
    }

    fs::create_dir_all(&path)?;

    if cmd.force {
        fs::remove_dir_all(&path)?
    }

    if cmd.no_copy {
        symlink_all(&curr, &path)?;
        println!(
            "Project linked to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );

    } else {
        copy_dir_all(&curr, &path)?;
        println!(
            "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
            path, namespace, name, version
        );
    }
    Ok(true)
}
