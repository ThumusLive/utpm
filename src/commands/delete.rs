use std::fs;

use toml::map::Map;
use tracing::instrument;
use typst_project::manifest::Manifest;

use crate::{
    manifest,
    utils::{
        paths::get_current_dir,
        specs::Extra,
        state::{Error, ErrorKind, Result},
    },
    write_manifest,
};

use super::DeleteArgs;

#[instrument]
pub fn run(cmd: &mut DeleteArgs) -> Result<bool> {
    let mut config = manifest!();

    if let Some(mut tool) = config.clone().tool {
        if let Some(ex) = tool.keys.get("utpm") {
            let mut extra: Extra = toml::from_str(toml::to_string(ex)?.as_str())?; //Todo: change this hack
            if let Some(mut dep) = extra.clone().dependencies {
                for e in &cmd.uri {
                    match dep.iter().position(|x| x == e) {
                        Some(val) => {
                            dep.remove(val);
                            println!("Removed");
                        }
                        None => println!("Can't remove it"),
                    };
                }
                extra.dependencies = Some(dep);
                tool.keys.insert("utpm".to_string(), Map::try_from(extra)?);
            }
        }
        config.tool = Some(tool);
    }

    write_manifest!(&config);

    Ok(true)
}
