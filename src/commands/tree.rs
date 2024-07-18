use owo_colors::OwoColorize;
use std::fs;

use crate::utils::{
    paths::d_packages,
    state::Result,
};

pub fn run() -> Result<bool> {
    let typ = d_packages();
    println!("{}", "Tree listing of your packages\n".bold());
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
