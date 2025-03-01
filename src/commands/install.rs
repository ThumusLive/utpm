use std::{env, fs, path::Path};

use crate::{
    commands::LinkArgs,
    load_manifest,
    utils::{
        copy_dir_all,
        paths::{
            check_path_dir, check_path_file, d_packages, datalocalutpm, get_current_dir,
            get_ssh_dir,
        },
        specs::Extra,
        state::{Error, ErrorKind, Result},
    },
};

use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use tracing::{debug, instrument};
use typst_project::{heuristics::MANIFEST_FILE, manifest::Manifest};

use super::{link, InstallArgs};

#[instrument]
pub fn run(cmd: &InstallArgs) -> Result<bool> {
    let path = format!("{}/tmp", datalocalutpm()?);
    if check_path_dir(&path) {
        fs::remove_dir_all(path)?;
    }
    init(cmd, 0)?;
    Ok(true)
}

#[instrument(skip(cmd))]
pub fn init(cmd: &InstallArgs, i: usize) -> Result<bool> {
    let path = if let Some(url) = &cmd.url {
        let dir = format!("{}/tmp/{}", datalocalutpm()?, i);
        debug!("url is set to {}, creating {}", url, dir);
        dir
    } else {
        let dir = get_current_dir()?;
        debug!("url is none, current dir: {}", dir);
        dir
    };

    if let Some(x) = &cmd.url {
        fs::create_dir_all(&path)?;
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
        if x.starts_with("git") || x.starts_with("http") {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_, username_from_url, _| {
                let binding = env::var("UTPM_USERNAME")
                    .unwrap_or(username_from_url.unwrap_or("git").to_string());
                let username = binding.as_str();
                match Cred::ssh_key_from_agent(username) {
                    Ok(cred) => Ok(cred),
                    Err(_) => Cred::ssh_key(
                        username,
                        None,
                        Path::new(&val),
                        Some(
                            env::var("UTPM_PASSPHRASE")
                                .unwrap_or(String::new())
                                .as_str(),
                        ),
                    ),
                }
            });

            let mut fo = FetchOptions::new();
            fo.remote_callbacks(callbacks);

            let mut builder = RepoBuilder::new();
            builder.fetch_options(fo);
            builder.clone(&x, Path::new(&path))?;
        } else {
            copy_dir_all(&x, &path)?;
        }
    };

    let typstfile = path.clone() + MANIFEST_FILE;
    if !check_path_file(&typstfile) {
        let origin = cmd.url.clone().unwrap_or("/".into());
        println!("{}", format!("x {}", origin));
        return Ok(false);
    }
    let file = load_manifest!(&path);
    let utpm = if let Some(value) = file.tool {
        value.get_section("utpm")?.unwrap_or(Extra::default())
    } else {
        Extra::default()
    };
    let namespace = utpm.namespace.unwrap_or("local".into());
    if check_path_dir(&format!(
        "{}/{}/{}/{}",
        d_packages()?,
        namespace,
        &file.package.name,
        &file.package.version
    )) {
        println!(
            "{}",
            format!("~ {}:{}", file.package.name, file.package.version)
        );
        return Ok(true);
    }

    println!("{}", format!("Installing {}...", file.package.name));
    if let Some(vec_depend) = utpm.dependencies {
        let mut y = 0;
        vec_depend
            .iter()
            .map(|a| -> Result<bool> {
                y += 1;
                let ins = InstallArgs {
                    force: cmd.force,
                    url: Some(a.to_string()),
                };
                init(&ins, i * vec_depend.len() + y)?;
                Ok(true)
            })
            .collect::<Result<Vec<bool>>>()?;
    }
    if !cmd.url.is_none() {
        let lnk = LinkArgs {
            force: cmd.force,
            no_copy: false,
        };
        link::run(&lnk, Some(path.clone()), false)?; //TODO: change here too
        fs::remove_dir_all(&path)?;
        println!(
            "{}",
            format!("+ {}:{}", file.package.name, file.package.version)
        );
    } else {
        println!("* Installation complete! If you want to use it as a lib, just do a `utpm link`!")
    }

    Ok(true)
}
