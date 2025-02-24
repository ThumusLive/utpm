use std::{
    env::{self, current_dir},
    fs::{self, read, read_dir, symlink_metadata},
    path::{self, Path},
    result::Result as R,
};

use dirs::cache_dir;

use super::state::{Error, ErrorKind, Result};

/// Get the path to your data directory.
/// Can be edited by using `UTPM_DATA_DIR` env. 
/// Used for getting your local packages.
pub fn get_data_dir() -> Result<String> {
    match env::var("UTPM_DATA_DIR") {
        Ok(str) => Ok(path::absolute(str)?.to_str().unwrap().to_string()),
        _ => match dirs::data_local_dir() {
            Some(dir) => match dir.to_str() {
                Some(string) => Ok(String::from(string)),
                None => Ok(String::from("/.local/share")), //default on linux
            },
            None => Ok(String::from("/.local/share")),
        },
    }
}

/// Get the path to your home directory.
/// Can be edited by using `UTPM_HOME_DIR` env.
/// Used for getting your ssh keys when running `publish` command.
pub fn get_home_dir() -> Result<String> {
    let err_hd = Error::empty(ErrorKind::HomeDir);
    match env::var("UTPM_HOME_DIR") {
        Ok(str) => Ok(path::absolute(str)?.to_str().unwrap().to_string()),
        _ => match dirs::home_dir() {
            Some(val) => match val.to_str() {
                Some(v) => Ok(String::from(v)),
                None => Err(err_hd),
            },
            None => Err(err_hd),
        },
    }
}

/// Get the path to your cache directory.
/// Can be edited by using `UTPM_CACHE_DIR` env.
/// Used for getting your downloaded packages from typst registry.
pub fn get_cache_dir() -> Result<String> {
    match env::var("UTPM_CACHE_DIR") {
        Ok(str) => Ok(path::absolute(str)?.to_str().unwrap().to_string()),
        _ => Ok(cache_dir()
            .unwrap_or("/.cache".into())
            .to_str()
            .unwrap_or("/.cache")
            .into()),
    }
}

/// Get the path to your ssh directory.
/// Can be edited by using `UTPM_SSH_DIR` env.
/// Used for getting your ssh keys for the `publish` command.
pub fn get_ssh_dir() -> Result<String> {
    match env::var("UTPM_SSH_DIR") {
        Ok(str) => Ok(path::absolute(str)?.to_str().unwrap().to_string()),
        _ => Ok(get_home_dir()? + "/.ssh"),
    }
}

/// Get the path to your downloaded packages.
pub fn c_packages() -> Result<String> {
    Ok(get_cache_dir()? + "/typst/packages")
}

/// Get the path to your local packages.
pub fn d_packages() -> Result<String> {
    Ok(get_data_dir()? + "/typst/packages")
}

/// Get the path to your utpm files.
/// Used to get and set your temporary files.
pub fn datalocalutpm() -> Result<String> {
    Ok(get_data_dir()? + "/utpm")
}


/// Get the path to your current directory.
/// Can be edited by using `UTPM_CURRENT_DIR` env.
/// Used to write and read yout `typst.toml` file.
pub fn get_current_dir() -> Result<String> {
    match env::var("UTPM_CURRENT_DIR") {
        Ok(str) => Ok(path::absolute(str)?.to_str().unwrap().to_string()),
        _ => match current_dir() {
            Ok(val) => match val.to_str() {
                Some(v) => Ok(String::from(v)),
                None => Err(Error::new(
                    ErrorKind::CurrentDir,
                    "There is no current directory.",
                )),
            },
            Err(val) => Err(Error::new(ErrorKind::CurrentDir, val.to_string())),
        },
    }
}

pub fn has_content(path: impl AsRef<Path>) -> Result<bool> {
    Ok(fs::read_dir(path)?.collect::<R<Vec<_>, _>>()?.len() > 0)
}

pub fn current_package() -> Result<String> {
    Ok(get_current_dir()? + "/typst.toml")
}

pub fn check_path_dir(path: impl AsRef<Path>) -> bool {
    read_dir(path).is_ok()
}

pub fn check_path_file(path: impl AsRef<Path>) -> bool {
    read(path).is_ok()
}

pub fn check_existing_symlink(path: impl AsRef<Path>) -> bool {
    let x = match symlink_metadata(path) {
        Ok(val) => val,
        _ => return false,
    };
    x.file_type().is_symlink()
}
