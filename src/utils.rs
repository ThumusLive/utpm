use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::create_dir_all;
use std::{fs, path::Path};

use std::{env, io, result::Result as R};

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use paths::{check_path_file, get_ssh_dir, has_content};
#[cfg(any(feature = "clone", feature = "publish", feature = "unlink"))]
use regex::Regex;
use state::{Error, ErrorKind};
use tracing::{error, info, instrument};
use typst_kit::download::{DownloadState, Progress};

pub mod macros;
pub mod paths;
pub mod specs;
pub mod state;

use self::state::Result;

/// Copy all subdirectories from a point to an other
/// From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
/// Edited to prepare a ci version
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() && entry.file_name() != ".utpm" && entry.file_name() != "install" {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Implementing a symlink function for all platform (unix version)
#[cfg(unix)]
pub fn symlink_all(
    origin: impl AsRef<Path>,
    new_path: impl AsRef<Path>,
) -> R<(), std::io::Error> {
    use std::os::unix::fs::symlink;
    symlink(origin, new_path)
}

/// Implementing a symlink function for all platform (windows version)
#[cfg(windows)]
pub fn symlink_all(
    origin: impl AsRef<Path>,
    new_path: impl AsRef<Path>,
) -> Result<(), std::io::Error> {
    use std::os::windows::fs::symlink_dir;
    symlink_dir(origin, new_path)
}

#[cfg(any(feature = "clone", feature = "publish", feature = "unlink"))]
pub fn regex_package() -> Regex {
    Regex::new(r"^@([a-z]+)\/([a-z]+(?:\-[a-z]+)?)\:(\d+)\.(\d+)\.(\d+)$").unwrap()
}
#[cfg(any(feature = "unlink"))]
pub fn regex_namespace() -> Regex {
    Regex::new(r"^@([a-z]+)$").unwrap()
}

#[cfg(any(feature = "unlink"))]
pub fn regex_packagename() -> Regex {
    Regex::new(r"^@([a-z]+)\/([a-z]+(?:\-[a-z]+)?)$").unwrap()
}

//todo: impl
pub struct ProgressPrint {}

impl Progress for ProgressPrint {
    fn print_start(&mut self) {}

    fn print_progress(&mut self, _state: &DownloadState) {}

    fn print_finish(&mut self, _state: &DownloadState) {}
}

#[instrument]
pub fn update_git_packages<P>(path_packages: P, url: &str) -> Result<Repository>
where
    P: AsRef<Path> + AsRef<OsStr> + Debug,
{
    create_dir_all(&path_packages)?;
    let repo: Repository;

    let sshpath = get_ssh_dir()?;
    let ed: String = sshpath.clone() + "/id_ed25519";
    let rsa: String = sshpath + "/id_rsa";
    let val: String = match env::var("UTPM_KEYPATH") {
        Ok(val) => val,
        Err(_) => {
            if check_path_file(&ed) {
                ed
            } else {
                rsa
            }
        }
    };

    info!(path = val);
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, username_from_url, _| {
        let binding: String =
            env::var("UTPM_USERNAME").unwrap_or(username_from_url.unwrap_or("git").to_string());
        let username: &str = binding.as_str();
        match Cred::ssh_key_from_agent(username) {
            Ok(cred) => Ok(cred),
            Err(_) => Ok(match env::var("UTPM_PASSPHRASE") {
                Ok(s) => {
                    info!(passphrase = true);
                    Cred::ssh_key(username, None, Path::new(&val), Some(s.as_str()))?
                }
                Err(_) => {
                    info!(passphrase = false);
                    Cred::ssh_key(username, None, Path::new(&val), None)?
                }
            }),
        }
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    if has_content(&path_packages)? {
        info!("Content found, starting a 'git pull origin main'");
        repo = Repository::open(path_packages)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], Some(&mut fo), None)?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            info!("up to date, nothing to do");
        } else if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", "main");
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
            info!("fast forward done");
        } else {
            error!("Can't rebase for now.");
            return Err(Error::empty(ErrorKind::UnknowError("todo".into())));
        }
    } else {
        info!("Start cloning");
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);
        repo = builder.clone(url, Path::new(&path_packages))?;
        info!("Package cloned");
    };
    Ok(repo)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn regex() {
        let re = regex_package();
        assert!(re.is_match("@preview/package:2.0.1"));
        assert!(!re.is_match("@preview/package-:2.0.1"));
        assert!(!re.is_match("@local/package-A:2.0.1"));
        assert!(re.is_match("@local/package-a:2.0.1"));
        assert!(!re.is_match("@local/p:1..1"));
        assert!(re.is_match("@a/p:1.0.1"));
        assert!(!re.is_match("@/p:1.0.1"));
        assert!(!re.is_match("p:1.0.1"));
        assert!(!re.is_match("@a/p"));
    }
}
