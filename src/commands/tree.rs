use owo_colors::OwoColorize;
use std::fs;
use tracing::instrument;

use crate::utils::{
    paths::{c_packages, d_packages},
    state::Result,
};

use super::ListTreeArgs;

use std::result::Result as R;

#[instrument]
pub fn run(cmd: &ListTreeArgs) -> Result<bool> {
    let typ: String = d_packages()?;
    println!("{}", "Tree listing of your packages\n".bold());
    if cmd.all {
        let preview: String = c_packages()?;
        read(typ)?;
        return read(preview);
    }

    if let Some(list) = &cmd.include {
        let preview: String = c_packages()?;
        for e in list {
            if e == "preview" {
                return read(preview);
            }
            match package_read(&format!("{}/local/{}", typ, e)) {
                Ok(_) => true,
                Err(_) => namespace_read(&format!("{}/{}", typ, e))?,
            };
        }
        Ok(true)
    } else {
        read(typ)
    }
}

fn read(typ: String) -> Result<bool> {
    let dirs = fs::read_dir(&typ)?;

    for dir_res in dirs {
        let dir = dir_res?;
        println!("@{}:", dir.file_name().to_str().unwrap().green().bold());
        let subupdirs = fs::read_dir(dir.path())?;

        for dir_res in subupdirs {
            let dir = dir_res?;
            println!("  {}:", dir.file_name().to_str().unwrap().green().bold());

            let subdirs = fs::read_dir(dir.path())?;
            for sub_dir_res in subdirs {
                let subdir = sub_dir_res?;
                println!("    - {}", subdir.file_name().to_str().unwrap().green());
            }
        }
    }
    Ok(true)
}

fn package_read(typ: &String) -> Result<bool> {
    let dirs = fs::read_dir(&typ)?;

    for dir_res in dirs {
        let dir = dir_res?;
        print!("{}", dir.file_name().to_str().unwrap());
    }
    println!();
    Ok(true)
}

fn namespace_read(typ: &String) -> Result<bool> {
    let dirs = fs::read_dir(&typ)?;

    for dir_res in dirs.into_iter().collect::<R<Vec<_>, _>>()? {
        println!("{}:", dir_res.file_name().into_string().unwrap());
        let subupdirs = fs::read_dir(dir_res.path())?;
        for dir_res in subupdirs.into_iter().collect::<R<Vec<_>, _>>()? {
            println!("  - {}", dir_res.file_name().to_str().unwrap());
        }
        println!();
    }
    Ok(true)
}
