use std::path::PathBuf;

use tracing::{debug, error, info, instrument, warn};
use typst_kit::{download::Downloader, package::PackageStorage};

use crate::{build, utils::ProgressPrint};

use crate::utils::{
    copy_dir_all,
    paths::{c_packages, check_path_dir, d_packages, get_current_dir, has_content},
    state::{Error, ErrorKind, Result},
    symlink_all,
};

use typst_syntax::package::{PackageSpec, PackageVersion};

use super::CloneArgs;

use regex::Regex;

#[instrument]
pub fn run(cmd: &CloneArgs) -> Result<bool> {
    let path: PathBuf = if let Some(path) = &cmd.path {
        path.clone()
    } else {
        get_current_dir()?.into()
    };
    if has_content(&path)? {
        debug!("found content");
        if cmd.force {
            warn!("force used, ignore content");
        } else {
            return Err(Error::new(
                ErrorKind::ContentFound,
                "Content found, cancelled",
            ));
        }
    }
    let re = Regex::new(r"@([a-z]+)\/([a-z\-]+)\:(\d+)\.(\d+)\.(\d+)").unwrap();
    let package: &String = &cmd.package;
    if let Some(cap) = re.captures(package) {
        let (_, [namespace, package, major, minor, patch]) = cap.extract();
        let val = format!(
            "{}/{namespace}/{package}/{major}.{minor}.{patch}",
            if namespace == "preview" {
                info!("preview found, cache dir use");
                c_packages()?
            } else {
                info!("no preview found, data dir use");
                d_packages()?
            }
        );
        if check_path_dir(&val) {
            if cmd.download_only {
                info!("download only, nothing to do.");
                return Ok(true);
            }
            if !cmd.redownload || namespace != "preview" {
                info!(
                    namespace = namespace,
                    redownload = cmd.redownload,
                    "Skip download..."
                );
                if cmd.symlink {
                    symlink_all(val, path)?;
                    info!("symlinked!");
                } else {
                    copy_dir_all(val, path)?;
                    info!("copied!");
                }
                return Ok(true);
            }
        }

        if cmd.redownload {}

        let pkg_sto = PackageStorage::new(
            Some(c_packages()?.into()),
            Some(d_packages()?.into()),
            Downloader::new(format!("utpm/{}", build::COMMIT_HASH)),
        );
        let printer = &mut ProgressPrint {};
        return match pkg_sto.prepare_package(
            &PackageSpec {
                namespace: namespace.into(),
                name: package.into(),
                version: PackageVersion {
                    major: major.parse::<u32>().unwrap(),
                    minor: minor.parse::<u32>().unwrap(),
                    patch: patch.parse::<u32>().unwrap(),
                },
            },
            printer,
        ) {
            Ok(val) => {
                info!(path = val.to_str().unwrap(), "package downloaded");
                if cmd.download_only {
                    debug!("download complete, nothing to do");
                    return Ok(true);
                }

                if cmd.symlink {
                    symlink_all(val, path)?;
                    info!("symlinked!");
                } else {
                    copy_dir_all(val, path)?;
                    info!("copied!");
                }

                Ok(true)
            }
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::PackageNotExist,
                    "This package doesn't exist. Verify on https://typst.app/universe to see if the package exist and/or the version is correct.",
                ));
            }
        };
    } else {
        error!("package not found, input: {}", package);
        return Err(Error::new(
            ErrorKind::PackageNotValid,
            "Can't extract your package. Example of a package: @namespace/package:1.0.0",
        ));
    }
}
