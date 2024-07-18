use std::{env, fs, path::Path};

use crate::{
    commands::LinkArgs,
    utils::{
        paths::{
            check_path_dir, check_path_file, d_packages, datalocalutpm, get_current_dir,
            get_ssh_dir,
        },
        specs::{Extra, TypstConfig},
        state::{Error, ErrorKind, Result},
    },
};
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};
use owo_colors::OwoColorize;

use super::{link, InstallArgs};

pub fn run(cmd: &InstallArgs) -> Result<bool> {
    let path = format!("{}/tmp", datalocalutpm());
    if check_path_dir(&path) {
        fs::remove_dir_all(path)?;
    }
    init(cmd, 0)?;
    Ok(true)
}

pub fn init(cmd: &InstallArgs, i: usize) -> Result<bool> {
    let path = if cmd.url.is_none() {
        get_current_dir()?
    } else {
        format!("{}/tmp/{}", datalocalutpm(), i)
    };

    if let Some(x) = &cmd.url {
        fs::create_dir_all(&path)?;
        let sshpath = get_ssh_dir()?;
        let ed = sshpath.clone() + "/id_ed25519";
        let rsa = sshpath + "/id_rsa";
        let val = if check_path_file(&ed) { ed } else { rsa };
        if x.starts_with("git") {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_, username_from_url, _| {
                match Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                    Ok(cred) => Ok(cred),
                    Err(_) => Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
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
            Repository::clone(&x, &path)?;
        }
    };

    let typstfile = path.clone() + "/typst.toml";
    if !check_path_file(&typstfile) {
        return Err(Error::empty(ErrorKind::ConfigFile));
    }

    let file = TypstConfig::load(&typstfile);
    let utpm = file.utpm;
    let namespace = utpm
        .clone()
        .unwrap_or(Extra::new(None, Some("local".to_string()), None))
        .namespace
        .unwrap_or("local".into());

    if check_path_dir(&format!(
        "{}/{}/{}/{}",
        d_packages(),
        namespace,
        &file.package.name,
        &file.package.version
    )) {
        println!(
            "{}",
            format!("~ {}:{}", file.package.name, file.package.version).bright_black()
        );
        return Ok(true);
    }

    println!("{}", format!("Installing {}...", file.package.name).bold());
    if let Some(fl) = utpm {
        if let Some(vec_depend) = fl.dependencies {
            let mut y = 0;
            let vec_of_dependencies = vec_depend
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
                .collect::<Vec<Result<bool>>>();

            for result_dependencies in vec_of_dependencies {
                result_dependencies?;
            }
        }
    }

    if !cmd.url.is_none() {
        let lnk = LinkArgs {
            force: cmd.force,
            no_copy: false,
        };

        link::run(&lnk, Some(path.clone()))?; //TODO: change here too
        fs::remove_dir_all(&path)?;
        println!(
            "{}",
            format!("+ {}:{}", file.package.name, file.package.version).bright_green()
        );
    } else {
        println!(
            "{}",
            "* Installation complete! If you want to use it as a lib, just do a `utpm link`!"
                .bold()
        )
    }

    Ok(true)
}
